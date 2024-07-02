///Register `RCC_ETHCKSELR` reader
pub type R = crate::R<RCC_ETHCKSELRrs>;
///Register `RCC_ETHCKSELR` writer
pub type W = crate::W<RCC_ETHCKSELRrs>;
///Field `ETHSRC` reader - ETHSRC
pub type ETHSRC_R = crate::FieldReader;
///Field `ETHSRC` writer - ETHSRC
pub type ETHSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ETHPTPDIV` reader - ETHPTPDIV
pub type ETHPTPDIV_R = crate::FieldReader;
///Field `ETHPTPDIV` writer - ETHPTPDIV
pub type ETHPTPDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - ETHSRC
    #[inline(always)]
    pub fn ethsrc(&self) -> ETHSRC_R {
        ETHSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:7 - ETHPTPDIV
    #[inline(always)]
    pub fn ethptpdiv(&self) -> ETHPTPDIV_R {
        ETHPTPDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ETHCKSELR")
            .field("ethsrc", &self.ethsrc())
            .field("ethptpdiv", &self.ethptpdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - ETHSRC
    #[inline(always)]
    #[must_use]
    pub fn ethsrc(&mut self) -> ETHSRC_W<RCC_ETHCKSELRrs> {
        ETHSRC_W::new(self, 0)
    }
    ///Bits 4:7 - ETHPTPDIV
    #[inline(always)]
    #[must_use]
    pub fn ethptpdiv(&mut self) -> ETHPTPDIV_W<RCC_ETHCKSELRrs> {
        ETHPTPDIV_W::new(self, 4)
    }
}
/**This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.

You can [`read`](crate::Reg::read) this register and get [`rcc_ethckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ethckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_ETHCKSELR)*/
pub struct RCC_ETHCKSELRrs;
impl crate::RegisterSpec for RCC_ETHCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ethckselr::R`](R) reader structure
impl crate::Readable for RCC_ETHCKSELRrs {}
///`write(|w| ..)` method takes [`rcc_ethckselr::W`](W) writer structure
impl crate::Writable for RCC_ETHCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_ETHCKSELR to value 0
impl crate::Resettable for RCC_ETHCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
