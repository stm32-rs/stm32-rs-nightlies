///Register `PLL1FRACR` reader
pub type R = crate::R<PLL1FRACRrs>;
///Register `PLL1FRACR` writer
pub type W = crate::W<PLL1FRACRrs>;
///Field `FRACV` reader - FRACV
pub type FRACV_R = crate::FieldReader<u16>;
///Field `FRACV` writer - FRACV
pub type FRACV_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `FRACLE` reader - FRACLE
pub type FRACLE_R = crate::BitReader;
///Field `FRACLE` writer - FRACLE
pub type FRACLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 3:15 - FRACV
    #[inline(always)]
    pub fn fracv(&self) -> FRACV_R {
        FRACV_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    ///Bit 16 - FRACLE
    #[inline(always)]
    pub fn fracle(&self) -> FRACLE_R {
        FRACLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1FRACR")
            .field("fracv", &self.fracv())
            .field("fracle", &self.fracle())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - FRACV
    #[inline(always)]
    pub fn fracv(&mut self) -> FRACV_W<'_, PLL1FRACRrs> {
        FRACV_W::new(self, 3)
    }
    ///Bit 16 - FRACLE
    #[inline(always)]
    pub fn fracle(&mut self) -> FRACLE_W<'_, PLL1FRACRrs> {
        FRACLE_W::new(self, 16)
    }
}
/**This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:PLL1FRACR)*/
pub struct PLL1FRACRrs;
impl crate::RegisterSpec for PLL1FRACRrs {
    type Ux = u32;
}
///`read()` method returns [`pll1fracr::R`](R) reader structure
impl crate::Readable for PLL1FRACRrs {}
///`write(|w| ..)` method takes [`pll1fracr::W`](W) writer structure
impl crate::Writable for PLL1FRACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1FRACR to value 0
impl crate::Resettable for PLL1FRACRrs {}
