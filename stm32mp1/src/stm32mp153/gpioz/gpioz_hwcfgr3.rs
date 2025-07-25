///Register `GPIOZ_HWCFGR3` reader
pub type R = crate::R<GPIOZ_HWCFGR3rs>;
///Field `ODR_RES` reader - ODR_RES
pub type ODR_RES_R = crate::FieldReader<u16>;
///Field `OTYPER_RES` reader - OTYPER_RES
pub type OTYPER_RES_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - ODR_RES
    #[inline(always)]
    pub fn odr_res(&self) -> ODR_RES_R {
        ODR_RES_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - OTYPER_RES
    #[inline(always)]
    pub fn otyper_res(&self) -> OTYPER_RES_R {
        OTYPER_RES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_HWCFGR3")
            .field("odr_res", &self.odr_res())
            .field("otyper_res", &self.otyper_res())
            .finish()
    }
}
/**GPIO hardware configuration register 3

You can [`read`](crate::Reg::read) this register and get [`gpioz_hwcfgr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_HWCFGR3)*/
pub struct GPIOZ_HWCFGR3rs;
impl crate::RegisterSpec for GPIOZ_HWCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_hwcfgr3::R`](R) reader structure
impl crate::Readable for GPIOZ_HWCFGR3rs {}
///`reset()` method sets GPIOZ_HWCFGR3 to value 0
impl crate::Resettable for GPIOZ_HWCFGR3rs {}
