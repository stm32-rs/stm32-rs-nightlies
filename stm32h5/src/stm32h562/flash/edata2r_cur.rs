#[doc = "Register `EDATA2R_CUR` reader"]
pub type R = crate::R<EDATA2R_CURrs>;
#[doc = "Field `EDATA2_STRT` reader - EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data"]
pub type EDATA2_STRT_R = crate::FieldReader;
#[doc = "Field `EDATA2_EN` reader - Bank2 flash high-cycle data enable"]
pub type EDATA2_EN_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data"]
    #[inline(always)]
    pub fn edata2_strt(&self) -> EDATA2_STRT_R {
        EDATA2_STRT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 15 - Bank2 flash high-cycle data enable"]
    #[inline(always)]
    pub fn edata2_en(&self) -> EDATA2_EN_R {
        EDATA2_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "FLASH data sectors configuration Bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edata2r_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EDATA2R_CURrs;
impl crate::RegisterSpec for EDATA2R_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edata2r_cur::R`](R) reader structure"]
impl crate::Readable for EDATA2R_CURrs {}
#[doc = "`reset()` method sets EDATA2R_CUR to value 0"]
impl crate::Resettable for EDATA2R_CURrs {
    const RESET_VALUE: u32 = 0;
}
