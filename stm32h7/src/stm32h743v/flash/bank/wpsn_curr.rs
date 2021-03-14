#[doc = "Reader of register WPSN_CURR"]
pub type R = crate::R<u32, super::WPSN_CURR>;
#[doc = "Reader of field `WRPSn`"]
pub type WRPSN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Bank 1 sector write protection option status byte"]
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
