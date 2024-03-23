#[doc = "Register `AXIMC_PERIPH_ID_3` reader"]
pub type R = crate::R<AXIMC_PERIPH_ID_3rs>;
#[doc = "Field `CUST_MOD_NUM` reader - CUST_MOD_NUM"]
pub type CUST_MOD_NUM_R = crate::FieldReader;
#[doc = "Field `REV_AND` reader - REV_AND"]
pub type REV_AND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - CUST_MOD_NUM"]
    #[inline(always)]
    pub fn cust_mod_num(&self) -> CUST_MOD_NUM_R {
        CUST_MOD_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - REV_AND"]
    #[inline(always)]
    pub fn rev_and(&self) -> REV_AND_R {
        REV_AND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXIMC peripheral ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_PERIPH_ID_3rs;
impl crate::RegisterSpec for AXIMC_PERIPH_ID_3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_periph_id_3::R`](R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_3rs {}
#[doc = "`reset()` method sets AXIMC_PERIPH_ID_3 to value 0"]
impl crate::Resettable for AXIMC_PERIPH_ID_3rs {
    const RESET_VALUE: u32 = 0;
}
