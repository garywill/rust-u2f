use slog;

#[derive(Serialize, Deserialize)]
pub enum SocketInput {
    CreateDeviceRequest(CreateDeviceRequest),
    Packet(Packet),
}

#[derive(Serialize, Deserialize)]
pub enum SocketOutput {
    CreateDeviceResponse(Result<DeviceDescription, CreateDeviceError>),
    Packet(Packet),
}

impl slog::Value for SocketOutput {
    fn serialize(
        &self,
        record: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        match self {
            &SocketOutput::CreateDeviceResponse(ref response) => slog::Value::serialize(
                &format!("CreateDeviceResponse({:?})", response),
                record,
                key,
                serializer,
            ),
            &SocketOutput::Packet { .. } => {
                slog::Value::serialize(&"Packet", record, key, serializer)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDeviceRequest;

impl slog::Value for CreateDeviceRequest {
    fn serialize(
        &self,
        record: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        slog::Value::serialize(&format!("{:?}", self), record, key, serializer)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceDescription {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CreateDeviceError {
    IOError,
    AlreadyExists,
    Closed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    bytes: Vec<u8>,
}

impl Packet {
    pub fn from_bytes(bytes: &[u8]) -> Packet {
        Packet {
            bytes: bytes.to_vec(),
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl slog::Value for Packet {
    fn serialize(
        &self,
        record: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        slog::Value::serialize(&format!("{:?}", self), record, key, serializer)
    }
}
