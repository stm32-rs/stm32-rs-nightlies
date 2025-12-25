///Register `TST_CFG` reader
pub type R = crate::R<TST_CFGrs>;
///Register `TST_CFG` writer
pub type W = crate::W<TST_CFGrs>;
///Field `TST_EN` reader - Test Enable
pub type TST_EN_R = crate::BitReader;
///Field `TST_EN` writer - Test Enable
pub type TST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TST_CFG` reader - DTB output selection
pub type TST_CFG_R = crate::FieldReader;
///Field `TST_CFG` writer - DTB output selection
pub type TST_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Test Enable
    #[inline(always)]
    pub fn tst_en(&self) -> TST_EN_R {
        TST_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - DTB output selection
    #[inline(always)]
    pub fn tst_cfg(&self) -> TST_CFG_R {
        TST_CFG_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TST_CFG")
            .field("tst_en", &self.tst_en())
            .field("tst_cfg", &self.tst_cfg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Test Enable
    #[inline(always)]
    pub fn tst_en(&mut self) -> TST_EN_W<'_, TST_CFGrs> {
        TST_EN_W::new(self, 0)
    }
    ///Bits 1:3 - DTB output selection
    #[inline(always)]
    pub fn tst_cfg(&mut self) -> TST_CFG_W<'_, TST_CFGrs> {
        TST_CFG_W::new(self, 1)
    }
}
/**LCSC Test Configuration Register

You can [`read`](crate::Reg::read) this register and get [`tst_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tst_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:TST_CFG)*/
pub struct TST_CFGrs;
impl crate::RegisterSpec for TST_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`tst_cfg::R`](R) reader structure
impl crate::Readable for TST_CFGrs {}
///`write(|w| ..)` method takes [`tst_cfg::W`](W) writer structure
impl crate::Writable for TST_CFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TST_CFG to value 0
impl crate::Resettable for TST_CFGrs {}
