///Register `PLL1DIVR1` reader
pub type R = crate::R<PLL1DIVR1rs>;
///Register `PLL1DIVR1` writer
pub type W = crate::W<PLL1DIVR1rs>;
///Field `DIVN` reader - multiplication factor for PLL1 VCO
pub type DIVN_R = crate::FieldReader<u16>;
///Field `DIVN` writer - multiplication factor for PLL1 VCO
pub type DIVN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DIVP` reader - PLL1 DIVP division factor
pub type DIVP_R = crate::FieldReader;
///Field `DIVP` writer - PLL1 DIVP division factor
pub type DIVP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DIVQ` reader - PLL1 DIVQ division factor
pub type DIVQ_R = crate::FieldReader;
///Field `DIVQ` writer - PLL1 DIVQ division factor
pub type DIVQ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:8 - multiplication factor for PLL1 VCO
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL1 DIVP division factor
    #[inline(always)]
    pub fn divp(&self) -> DIVP_R {
        DIVP_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor
    #[inline(always)]
    pub fn divq(&self) -> DIVQ_R {
        DIVQ_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1DIVR1")
            .field("divn", &self.divn())
            .field("divp", &self.divp())
            .field("divq", &self.divq())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - multiplication factor for PLL1 VCO
    #[inline(always)]
    pub fn divn(&mut self) -> DIVN_W<'_, PLL1DIVR1rs> {
        DIVN_W::new(self, 0)
    }
    ///Bits 9:15 - PLL1 DIVP division factor
    #[inline(always)]
    pub fn divp(&mut self) -> DIVP_W<'_, PLL1DIVR1rs> {
        DIVP_W::new(self, 9)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor
    #[inline(always)]
    pub fn divq(&mut self) -> DIVQ_W<'_, PLL1DIVR1rs> {
        DIVQ_W::new(self, 16)
    }
}
/**RCC PLL1 dividers configuration register 1

You can [`read`](crate::Reg::read) this register and get [`pll1divr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:PLL1DIVR1)*/
pub struct PLL1DIVR1rs;
impl crate::RegisterSpec for PLL1DIVR1rs {
    type Ux = u32;
}
///`read()` method returns [`pll1divr1::R`](R) reader structure
impl crate::Readable for PLL1DIVR1rs {}
///`write(|w| ..)` method takes [`pll1divr1::W`](W) writer structure
impl crate::Writable for PLL1DIVR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1DIVR1 to value 0x0101_0280
impl crate::Resettable for PLL1DIVR1rs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
