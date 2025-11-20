use crate::boxes::FourCC;

/// Typed view over common MP4 / ISOBMFF boxes.
///
/// Anything not in this list becomes `KnownBox::Unknown(fourcc)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnownBox {
    // File-level / top-level
    Ftyp,
    Moov,
    Mdat,
    Free,
    Skip,
    Wide,
    Meta,
    Pssh,
    Sidx,
    Ssix,
    Prft,
    Styp,
    Emsg,
    Mfra,
    Mfro,

    // moov children
    Mvhd,
    Trak,
    Mvex,
    Udta,

    // trak children
    Tkhd,
    Edts,
    Mdia,
    Tref,
    Iprp,
    Meco,

    // edts children
    Elst,

    // mdia children
    Mdhd,
    Hdlr,
    Minf,

    // minf children
    Vmhd,
    Smhd,
    Hmhd,
    Nmhd,
    Dinf,
    Stbl,

    // dinf children
    Dref,

    // stbl children
    Stsd,
    Stts,
    Ctts,
    Stsc,
    Stsz,
    Stz2,
    Stco,
    Co64,
    Stss,
    Stsh,
    Padb,
    Stdp,
    Sdtp,
    Sgpd,
    Sbgp,
    Subs,

    // fragmented / mvex / moof / traf
    MvexBox,
    Mehd,
    Trex,
    Moof,
    Mfhd,
    Traf,
    Tfhd,
    Tfdt,
    Trun,
    Tfra,

    // meta / HEIF-ish
    Iloc,
    Iinf,
    Infe,
    Iref,
    Ipco,
    Ipma,
    Ipci,
    Ispe,
    Pixi,
    AuxC,
    Clap,
    Colr,
    Hvcc,
    Avcc,
    Pitm,

    // Encryption / CENC
    Sinf,
    Schm,
    Schi,
    Tenc,
    Saio,
    Saiz,
    Senc,
    Frma,

    // Sample entries (video)
    Avc1,
    Avc2,
    Avc3,
    Avc4,
    Hev1,
    Hvc1,
    Vvc1,
    Mp4v,
    Vp08,
    Vp09,
    Av01,

    // Sample entries (audio)
    Mp4a,
    Ac3,
    Ec3,
    Opus,
    Samr,
    Sawb,
    Alac,
    Flac,

    // Misc / QT-ish / common extras
    Pasp,
    Cslg,
    Cprt,
    Gama,
    Fiel,
    Tapt,

    // Raw UUID/vendor
    Uuid,

    // Anything else
    Unknown(FourCC),
}

impl From<FourCC> for KnownBox {
    fn from(cc: FourCC) -> Self {
        match &cc.0 {
            b"ftyp" => KnownBox::Ftyp,
            b"moov" => KnownBox::Moov,
            b"mdat" => KnownBox::Mdat,
            b"free" => KnownBox::Free,
            b"skip" => KnownBox::Skip,
            b"wide" => KnownBox::Wide,
            b"meta" => KnownBox::Meta,
            b"pssh" => KnownBox::Pssh,
            b"sidx" => KnownBox::Sidx,
            b"ssix" => KnownBox::Ssix,
            b"prft" => KnownBox::Prft,
            b"styp" => KnownBox::Styp,
            b"emsg" => KnownBox::Emsg,
            b"mfra" => KnownBox::Mfra,
            b"mfro" => KnownBox::Mfro,

            b"mvhd" => KnownBox::Mvhd,
            b"trak" => KnownBox::Trak,
            b"mvex" => KnownBox::Mvex,
            b"udta" => KnownBox::Udta,

            b"tkhd" => KnownBox::Tkhd,
            b"edts" => KnownBox::Edts,
            b"mdia" => KnownBox::Mdia,
            b"tref" => KnownBox::Tref,
            b"iprp" => KnownBox::Iprp,
            b"meco" => KnownBox::Meco,

            b"elst" => KnownBox::Elst,

            b"mdhd" => KnownBox::Mdhd,
            b"hdlr" => KnownBox::Hdlr,
            b"minf" => KnownBox::Minf,

            b"vmhd" => KnownBox::Vmhd,
            b"smhd" => KnownBox::Smhd,
            b"hmhd" => KnownBox::Hmhd,
            b"nmhd" => KnownBox::Nmhd,
            b"dinf" => KnownBox::Dinf,
            b"stbl" => KnownBox::Stbl,

            b"dref" => KnownBox::Dref,

            b"stsd" => KnownBox::Stsd,
            b"stts" => KnownBox::Stts,
            b"ctts" => KnownBox::Ctts,
            b"stsc" => KnownBox::Stsc,
            b"stsz" => KnownBox::Stsz,
            b"stz2" => KnownBox::Stz2,
            b"stco" => KnownBox::Stco,
            b"co64" => KnownBox::Co64,
            b"stss" => KnownBox::Stss,
            b"stsh" => KnownBox::Stsh,
            b"padb" => KnownBox::Padb,
            b"stdp" => KnownBox::Stdp,
            b"sdtp" => KnownBox::Sdtp,
            b"sgpd" => KnownBox::Sgpd,
            b"sbgp" => KnownBox::Sbgp,
            b"subs" => KnownBox::Subs,

            b"mehd" => KnownBox::Mehd,
            b"trex" => KnownBox::Trex,
            b"moof" => KnownBox::Moof,
            b"mfhd" => KnownBox::Mfhd,
            b"traf" => KnownBox::Traf,
            b"tfhd" => KnownBox::Tfhd,
            b"tfdt" => KnownBox::Tfdt,
            b"trun" => KnownBox::Trun,
            b"tfra" => KnownBox::Tfra,

            b"iloc" => KnownBox::Iloc,
            b"iinf" => KnownBox::Iinf,
            b"infe" => KnownBox::Infe,
            b"iref" => KnownBox::Iref,
            b"ipco" => KnownBox::Ipco,
            b"ipma" => KnownBox::Ipma,
            b"ipci" => KnownBox::Ipci,
            b"ispe" => KnownBox::Ispe,
            b"pixi" => KnownBox::Pixi,
            b"auxC" => KnownBox::AuxC,
            b"clap" => KnownBox::Clap,
            b"colr" => KnownBox::Colr,
            b"hvcC" => KnownBox::Hvcc,
            b"avcC" => KnownBox::Avcc,
            b"pitm" => KnownBox::Pitm,

            b"sinf" => KnownBox::Sinf,
            b"schm" => KnownBox::Schm,
            b"schi" => KnownBox::Schi,
            b"tenc" => KnownBox::Tenc,
            b"saio" => KnownBox::Saio,
            b"saiz" => KnownBox::Saiz,
            b"senc" => KnownBox::Senc,
            b"frma" => KnownBox::Frma,

            b"avc1" => KnownBox::Avc1,
            b"avc2" => KnownBox::Avc2,
            b"avc3" => KnownBox::Avc3,
            b"avc4" => KnownBox::Avc4,
            b"hev1" => KnownBox::Hev1,
            b"hvc1" => KnownBox::Hvc1,
            b"vvc1" => KnownBox::Vvc1,
            b"mp4v" => KnownBox::Mp4v,
            b"vp08" => KnownBox::Vp08,
            b"vp09" => KnownBox::Vp09,
            b"av01" => KnownBox::Av01,

            b"mp4a" => KnownBox::Mp4a,
            b"ac-3" => KnownBox::Ac3,
            b"ec-3" => KnownBox::Ec3,
            b"opus" => KnownBox::Opus,
            b"samr" => KnownBox::Samr,
            b"sawb" => KnownBox::Sawb,
            b"alac" => KnownBox::Alac,
            b"flac" => KnownBox::Flac,

            b"pasp" => KnownBox::Pasp,
            b"cslg" => KnownBox::Cslg,
            b"cprt" => KnownBox::Cprt,
            b"gama" => KnownBox::Gama,
            b"fiel" => KnownBox::Fiel,
            b"tapt" => KnownBox::Tapt,

            b"uuid" => KnownBox::Uuid,

            _ => KnownBox::Unknown(cc),        }
    }
}

impl KnownBox {
    /// Does this box *contain* child boxes (container semantics)?
    pub fn is_container(&self) -> bool {
        matches!(
            self,
            KnownBox::Moov
                | KnownBox::Trak
                | KnownBox::Mdia
                | KnownBox::Minf
                | KnownBox::Stbl
                | KnownBox::Edts
                | KnownBox::Udta
                | KnownBox::Meta
                | KnownBox::Moof
                | KnownBox::Mvex
                | KnownBox::Mfra
                | KnownBox::Meco
                | KnownBox::Traf
                | KnownBox::Sinf
                | KnownBox::Iprp
                | KnownBox::Iref
                | KnownBox::Ipco
                | KnownBox::Ipma
        )
    }

    /// Is this a FullBox (version + flags)?
    pub fn is_full_box(&self) -> bool {
        matches!(
            self,
            KnownBox::Mvhd
                | KnownBox::Tkhd
                | KnownBox::Mdhd
                | KnownBox::Hdlr
                | KnownBox::Vmhd
                | KnownBox::Smhd
                | KnownBox::Nmhd
                | KnownBox::Dref
                | KnownBox::Stts
                | KnownBox::Ctts
                | KnownBox::Stsc
                | KnownBox::Stsz
                | KnownBox::Stz2
                | KnownBox::Stco
                | KnownBox::Co64
                | KnownBox::Stss
                | KnownBox::Stsh
                | KnownBox::Padb
                | KnownBox::Stdp
                | KnownBox::Sdtp
                | KnownBox::Sgpd
                | KnownBox::Sbgp
                | KnownBox::Subs
                | KnownBox::Elst
                | KnownBox::Sidx
                | KnownBox::Mehd
                | KnownBox::Trex
                | KnownBox::Mfhd
                | KnownBox::Tfhd
                | KnownBox::Tfdt
                | KnownBox::Trun
                | KnownBox::Tfra
                | KnownBox::Iloc
                | KnownBox::Iinf
                | KnownBox::Infe
                | KnownBox::Pitm
                | KnownBox::Pssh
                | KnownBox::Schi
                | KnownBox::Saio
                | KnownBox::Saiz
        )
    }
}
