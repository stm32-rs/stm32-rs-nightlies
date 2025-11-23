///Register `AHBSRAM1CR` reader
pub type R = crate::R<AHBSRAM1CRrs>;
///Register `AHBSRAM1CR` writer
pub type W = crate::W<AHBSRAM1CRrs>;
///Field `SRAMER` reader - SRAM erase
pub type SRAMER_R = crate::BitReader;
///Field `SRAMER` writer - SRAM erase
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBSRAM1CR")
            .field("sramer", &self.sramer())
            .finish()
    }
}
impl W {
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&mut self) -> SRAMER_W<'_, AHBSRAM1CRrs> {
        SRAMER_W::new(self, 8)
    }
}
/**RAMCFG AHBSRAM1 control register

You can [`read`](crate::Reg::read) this register and get [`ahbsram1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsram1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RAMCFG:AHBSRAM1CR)*/
pub struct AHBSRAM1CRrs;
impl crate::RegisterSpec for AHBSRAM1CRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbsram1cr::R`](R) reader structure
impl crate::Readable for AHBSRAM1CRrs {}
///`write(|w| ..)` method takes [`ahbsram1cr::W`](W) writer structure
impl crate::Writable for AHBSRAM1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBSRAM1CR to value 0
impl crate::Resettable for AHBSRAM1CRrs {}
