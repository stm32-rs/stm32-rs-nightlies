///Register `PLL4FRACR` reader
pub type R = crate::R<PLL4FRACRrs>;
///Register `PLL4FRACR` writer
pub type W = crate::W<PLL4FRACRrs>;
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
        f.debug_struct("PLL4FRACR")
            .field("fracv", &self.fracv())
            .field("fracle", &self.fracle())
            .finish()
    }
}
impl W {
    ///Bits 3:15 - FRACV
    #[inline(always)]
    pub fn fracv(&mut self) -> FRACV_W<'_, PLL4FRACRrs> {
        FRACV_W::new(self, 3)
    }
    ///Bit 16 - FRACLE
    #[inline(always)]
    pub fn fracle(&mut self) -> FRACLE_W<'_, PLL4FRACRrs> {
        FRACLE_W::new(self, 16)
    }
}
/**This register is used to fine-tune the frequency of the PLL4 VCO.

You can [`read`](crate::Reg::read) this register and get [`pll4fracr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4fracr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:PLL4FRACR)*/
pub struct PLL4FRACRrs;
impl crate::RegisterSpec for PLL4FRACRrs {
    type Ux = u32;
}
///`read()` method returns [`pll4fracr::R`](R) reader structure
impl crate::Readable for PLL4FRACRrs {}
///`write(|w| ..)` method takes [`pll4fracr::W`](W) writer structure
impl crate::Writable for PLL4FRACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL4FRACR to value 0
impl crate::Resettable for PLL4FRACRrs {}
