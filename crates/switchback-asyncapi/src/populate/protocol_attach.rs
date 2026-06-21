//! Kafka / AMQP / MQTT protocol attachment helpers.

use serde_json::Value;
use switchback_codec_pb::canardleteer::switchback::protocol::amqp::v1alpha1::{
    AmqpChannelMeta, AmqpContractMeta, AmqpMessageMeta, AmqpOperationMeta,
};
use switchback_codec_pb::canardleteer::switchback::protocol::kafka::v1alpha1::{
    KafkaChannelMeta, KafkaContractMeta, KafkaMessageMeta, KafkaOperationMeta,
};
use switchback_codec_pb::canardleteer::switchback::protocol::mqtt::v1alpha1::{
    MqttChannelMeta, MqttContractMeta, MqttMessageMeta, MqttOperationMeta,
};
use switchback_protocols::{AmqpProtocol, KafkaProtocol, MqttProtocol};
use switchback_traits::ProtocolAttachment;

pub fn contract_attachments(root: &Value) -> Vec<ProtocolAttachment> {
    let mut out = Vec::new();
    out.extend(kafka_contract(root));
    out.extend(amqp_contract(root));
    out.extend(mqtt_contract(root));
    out
}

pub fn bindings_protocols(bindings: &Value) -> Vec<ProtocolAttachment> {
    let mut out = Vec::new();
    if let Some(kafka) = bindings.get("kafka") {
        out.extend(kafka_channel(kafka));
        out.extend(kafka_operation(kafka));
        out.extend(kafka_message(kafka));
    }
    if let Some(amqp) = bindings.get("amqp") {
        out.extend(amqp_channel(amqp));
        out.extend(amqp_operation(amqp));
        out.extend(amqp_message(amqp));
    }
    if let Some(mqtt) = bindings.get("mqtt") {
        out.extend(mqtt_channel(mqtt));
        out.extend(mqtt_operation(mqtt));
        out.extend(mqtt_message(mqtt));
    }
    out
}

fn kafka_contract(root: &Value) -> Vec<ProtocolAttachment> {
    let servers: Vec<String> = root
        .get("servers")
        .and_then(collect_server_urls)
        .unwrap_or_default();
    if servers.is_empty() {
        return Vec::new();
    }
    let kafka = KafkaProtocol;
    vec![kafka.attach_contract(&KafkaContractMeta {
        bootstrap_servers: servers,
        ..Default::default()
    })]
}

fn amqp_contract(root: &Value) -> Vec<ProtocolAttachment> {
    let vhost = root
        .get("servers")
        .and_then(|s| s.as_object())
        .and_then(|map| map.values().next())
        .and_then(|srv| srv.get("bindings"))
        .and_then(|b| b.get("amqp"))
        .and_then(|a| a.get("defaultVhost"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if vhost.is_empty() {
        return Vec::new();
    }
    let amqp = AmqpProtocol;
    vec![amqp.attach_contract(&AmqpContractMeta {
        default_vhost: vhost,
        ..Default::default()
    })]
}

fn mqtt_contract(root: &Value) -> Vec<ProtocolAttachment> {
    let urls = root
        .get("servers")
        .and_then(collect_server_urls)
        .unwrap_or_default();
    if urls.is_empty() {
        return Vec::new();
    }
    let mqtt = MqttProtocol;
    vec![mqtt.attach_contract(&MqttContractMeta {
        broker_urls: urls,
        ..Default::default()
    })]
}

fn collect_server_urls(servers: &Value) -> Option<Vec<String>> {
    match servers {
        Value::Array(items) => Some(
            items
                .iter()
                .filter_map(|s| s.get("url").and_then(|u| u.as_str()).map(str::to_string))
                .collect(),
        ),
        Value::Object(map) => Some(map.keys().map(ToString::to_string).collect()),
        _ => None,
    }
}

fn kafka_channel(binding: &Value) -> Vec<ProtocolAttachment> {
    let topic = binding
        .get("topic")
        .or_else(|| binding.get("channels").and_then(|c| c.as_object()?.values().next()))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if topic.is_empty() && binding.get("partitions").is_none() {
        return Vec::new();
    }
    let kafka = KafkaProtocol;
    vec![kafka.attach_channel(&KafkaChannelMeta {
        topic,
        partitions: binding
            .get("partitions")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32,
        replicas: binding
            .get("replicas")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32,
        ..Default::default()
    })]
}

fn kafka_operation(binding: &Value) -> Vec<ProtocolAttachment> {
    let group_id = binding
        .get("groupId")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let client_id = binding
        .get("clientId")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if group_id.is_empty() && client_id.is_empty() {
        return Vec::new();
    }
    let kafka = KafkaProtocol;
    vec![kafka.attach_operation(&KafkaOperationMeta {
        group_id,
        client_id,
        ..Default::default()
    })]
}

fn kafka_message(binding: &Value) -> Vec<ProtocolAttachment> {
    let schema_id_location = binding
        .get("schemaIdLocation")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if schema_id_location.is_empty() {
        return Vec::new();
    }
    let kafka = KafkaProtocol;
    vec![kafka.attach_message(&KafkaMessageMeta {
        schema_id_location,
        schema_id_payload_encoding: binding
            .get("schemaIdPayloadEncoding")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        schema_lookup_strategy: binding
            .get("schemaLookupStrategy")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        ..Default::default()
    })]
}

fn amqp_channel(binding: &Value) -> Vec<ProtocolAttachment> {
    let is = binding
        .get("is")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if is.is_empty()
        && binding.get("exchange").is_none()
        && binding.get("queue").is_none()
    {
        return Vec::new();
    }
    let amqp = AmqpProtocol;
    let exchange = binding.get("exchange");
    vec![amqp.attach_channel(&AmqpChannelMeta {
        channel_kind: is,
        exchange_name: exchange
            .and_then(|e| e.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        exchange_type: exchange
            .and_then(|e| e.get("type"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        exchange_durable: exchange
            .and_then(|e| e.get("durable"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        queue_name: binding
            .get("queue")
            .and_then(|q| q.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        ..Default::default()
    })]
}

fn amqp_operation(binding: &Value) -> Vec<ProtocolAttachment> {
    let delivery_mode = binding
        .get("deliveryMode")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    let priority = binding
        .get("priority")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    if delivery_mode == 0 && priority == 0 {
        return Vec::new();
    }
    let amqp = AmqpProtocol;
    vec![amqp.attach_operation(&AmqpOperationMeta {
        delivery_mode,
        priority,
        ..Default::default()
    })]
}

fn amqp_message(binding: &Value) -> Vec<ProtocolAttachment> {
    let content_type = binding
        .get("contentType")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if content_type.is_empty() {
        return Vec::new();
    }
    let amqp = AmqpProtocol;
    vec![amqp.attach_message(&AmqpMessageMeta {
        content_type,
        ..Default::default()
    })]
}

fn mqtt_channel(binding: &Value) -> Vec<ProtocolAttachment> {
    let topic = binding
        .get("topic")
        .or_else(|| binding.get("channel"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if topic.is_empty() {
        return Vec::new();
    }
    let mqtt = MqttProtocol;
    vec![mqtt.attach_channel(&MqttChannelMeta {
        topic,
        ..Default::default()
    })]
}

fn mqtt_operation(binding: &Value) -> Vec<ProtocolAttachment> {
    let qos = binding.get("qos").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
    let retain = binding
        .get("retain")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if qos == 0 && !retain {
        return Vec::new();
    }
    let mqtt = MqttProtocol;
    vec![mqtt.attach_operation(&MqttOperationMeta {
        qos,
        retain,
        message_expiry_interval: binding
            .get("messageExpiryInterval")
            .and_then(|v| v.as_u64())
            .unwrap_or(0),
        ..Default::default()
    })]
}

fn mqtt_message(binding: &Value) -> Vec<ProtocolAttachment> {
    let content_type = binding
        .get("contentType")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let response_topic = binding
        .get("responseTopic")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if content_type.is_empty() && response_topic.is_empty() {
        return Vec::new();
    }
    let mqtt = MqttProtocol;
    vec![mqtt.attach_message(&MqttMessageMeta {
        content_type,
        response_topic,
        payload_format_indicator: binding
            .get("payloadFormatIndicator")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32,
        ..Default::default()
    })]
}
