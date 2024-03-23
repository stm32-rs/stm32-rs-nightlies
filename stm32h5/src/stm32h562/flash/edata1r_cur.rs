#[doc = "Register `EDATA1R_CUR` reader"]
pub type R = crate::R<EDATA1R_CURrs>;
#[doc = "Field `EDATA1_STRT` reader - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ..."]
pub type EDATA1_STRT_R = crate::FieldReader;
#[doc = "Field `EDATA1_EN` reader - Bank1 flash high-cycle data enable"]
pub type EDATA1_EN_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ..."]
    #[inline(always)]
    pub fn edata1_strt(&self) -> EDATA1_STRT_R {
        EDATA1_STRT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 15 - Bank1 flash high-cycle data enable"]
    #[inline(always)]
    pub fn edata1_en(&self) -> EDATA1_EN_R {
        EDATA1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "FLASH data sector configuration Bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edata1r_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EDATA1R_CURrs;
impl crate::RegisterSpec for EDATA1R_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edata1r_cur::R`](R) reader structure"]
impl crate::Readable for EDATA1R_CURrs {}
#[doc = "`reset()` method sets EDATA1R_CUR to value 0"]
impl crate::Resettable for EDATA1R_CURrs {
    const RESET_VALUE: u32 = 0;
}
