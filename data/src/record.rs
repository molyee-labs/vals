pub type binary = Vec<u8>;

pub enum AudioFormat {
    FLAC,
    WavPack,
    G729,
    GSM_FR,
    CELT,
    Speex,
}
pub enum VideoFormat {
    H264,
    VP8,
    VP9,
    MP4
}

pub struct Record {
    owner: UserId,
    miner: UserId,
    time: f64,
    data: Data,
    source_id: Uuid,
    approved: f64,
}

pub enum Data {
    Text(String),
    Audio(binary, AudioFormat),
    Video(binary, VideoFormat)
}

pub struct Source {
    id: Uuid,
    gid: Uuid,
    language: i32,
    data: Data
}