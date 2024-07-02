///Register `ITLINE8` reader
pub type R = crate::R<ITLINE8rs>;
///Field `UCPD1` reader - UCPD1
pub type UCPD1_R = crate::BitReader;
///Field `UCPD2` reader - UCPD2
pub type UCPD2_R = crate::BitReader;
impl R {
    ///Bit 0 - UCPD1
    #[inline(always)]
    pub fn ucpd1(&self) -> UCPD1_R {
        UCPD1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UCPD2
    #[inline(always)]
    pub fn ucpd2(&self) -> UCPD2_R {
        UCPD2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE8")
            .field("ucpd1", &self.ucpd1())
            .field("ucpd2", &self.ucpd2())
            .finish()
    }
}
/**interrupt line 8 status register

You can [`read`](crate::Reg::read) this register and get [`itline8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#SYSCFG_VREFBUF:ITLINE8)*/
pub struct ITLINE8rs;
impl crate::RegisterSpec for ITLINE8rs {
    type Ux = u32;
}
///`read()` method returns [`itline8::R`](R) reader structure
impl crate::Readable for ITLINE8rs {}
///`reset()` method sets ITLINE8 to value 0
impl crate::Resettable for ITLINE8rs {
    const RESET_VALUE: u32 = 0;
}
