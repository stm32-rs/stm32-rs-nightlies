///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `SWP_TBYP` reader - SWP transceiver bypass
pub type SWP_TBYP_R = crate::BitReader;
///Field `SWP_TBYP` writer - SWP transceiver bypass
pub type SWP_TBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWP_CLASS` reader - SWP class selection
pub type SWP_CLASS_R = crate::BitReader;
///Field `SWP_CLASS` writer - SWP class selection
pub type SWP_CLASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SWP transceiver bypass
    #[inline(always)]
    pub fn swp_tbyp(&self) -> SWP_TBYP_R {
        SWP_TBYP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SWP class selection
    #[inline(always)]
    pub fn swp_class(&self) -> SWP_CLASS_R {
        SWP_CLASS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("swp_tbyp", &self.swp_tbyp())
            .field("swp_class", &self.swp_class())
            .finish()
    }
}
impl W {
    ///Bit 0 - SWP transceiver bypass
    #[inline(always)]
    pub fn swp_tbyp(&mut self) -> SWP_TBYP_W<'_, ORrs> {
        SWP_TBYP_W::new(self, 0)
    }
    ///Bit 1 - SWP class selection
    #[inline(always)]
    pub fn swp_class(&mut self) -> SWP_CLASS_W<'_, ORrs> {
        SWP_CLASS_W::new(self, 1)
    }
}
/**SWPMI Option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#SWPMI:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
