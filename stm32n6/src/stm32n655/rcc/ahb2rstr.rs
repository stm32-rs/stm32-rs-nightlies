///Register `AHB2RSTR` reader
pub type R = crate::R<AHB2RSTRrs>;
///Register `AHB2RSTR` writer
pub type W = crate::W<AHB2RSTRrs>;
///Field `RAMCFGRST` reader - RAMCFG reset
pub type RAMCFGRST_R = crate::BitReader;
///Field `RAMCFGRST` writer - RAMCFG reset
pub type RAMCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MDF1RST` reader - MDF1 reset
pub type MDF1RST_R = crate::BitReader;
///Field `MDF1RST` writer - MDF1 reset
pub type MDF1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1RST` reader - ADF1 reset
pub type ADF1RST_R = crate::BitReader;
///Field `ADF1RST` writer - ADF1 reset
pub type ADF1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 12 - RAMCFG reset
    #[inline(always)]
    pub fn ramcfgrst(&self) -> RAMCFGRST_R {
        RAMCFGRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - MDF1 reset
    #[inline(always)]
    pub fn mdf1rst(&self) -> MDF1RST_R {
        MDF1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ADF1 reset
    #[inline(always)]
    pub fn adf1rst(&self) -> ADF1RST_R {
        ADF1RST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2RSTR")
            .field("ramcfgrst", &self.ramcfgrst())
            .field("mdf1rst", &self.mdf1rst())
            .field("adf1rst", &self.adf1rst())
            .finish()
    }
}
impl W {
    ///Bit 12 - RAMCFG reset
    #[inline(always)]
    pub fn ramcfgrst(&mut self) -> RAMCFGRST_W<'_, AHB2RSTRrs> {
        RAMCFGRST_W::new(self, 12)
    }
    ///Bit 16 - MDF1 reset
    #[inline(always)]
    pub fn mdf1rst(&mut self) -> MDF1RST_W<'_, AHB2RSTRrs> {
        MDF1RST_W::new(self, 16)
    }
    ///Bit 17 - ADF1 reset
    #[inline(always)]
    pub fn adf1rst(&mut self) -> ADF1RST_W<'_, AHB2RSTRrs> {
        ADF1RST_W::new(self, 17)
    }
}
/**RCC AHB2 reset register

You can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:AHB2RSTR)*/
pub struct AHB2RSTRrs;
impl crate::RegisterSpec for AHB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2rstr::R`](R) reader structure
impl crate::Readable for AHB2RSTRrs {}
///`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure
impl crate::Writable for AHB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTRrs {}
