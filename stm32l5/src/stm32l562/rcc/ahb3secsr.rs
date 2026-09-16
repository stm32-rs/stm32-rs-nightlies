///Register `AHB3SECSR` reader
pub type R = crate::R<AHB3SECSRrs>;
///Field `FSMCSECF` reader - FSMCSECF
pub type FSMCSECF_R = crate::BitReader;
///Field `OSPI1SECF` reader - OSPI1SECF
pub type OSPI1SECF_R = crate::BitReader;
impl R {
    ///Bit 0 - FSMCSECF
    #[inline(always)]
    pub fn fsmcsecf(&self) -> FSMCSECF_R {
        FSMCSECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - OSPI1SECF
    #[inline(always)]
    pub fn ospi1secf(&self) -> OSPI1SECF_R {
        OSPI1SECF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3SECSR")
            .field("ospi1secf", &self.ospi1secf())
            .field("fsmcsecf", &self.fsmcsecf())
            .finish()
    }
}
/**RCC AHB3 security status register

You can [`read`](crate::Reg::read) this register and get [`ahb3secsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:AHB3SECSR)*/
pub struct AHB3SECSRrs;
impl crate::RegisterSpec for AHB3SECSRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3secsr::R`](R) reader structure
impl crate::Readable for AHB3SECSRrs {}
///`reset()` method sets AHB3SECSR to value 0
impl crate::Resettable for AHB3SECSRrs {}
