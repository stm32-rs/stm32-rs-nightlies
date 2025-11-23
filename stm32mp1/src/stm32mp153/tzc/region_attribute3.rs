///Register `REGION_ATTRIBUTE3` reader
pub type R = crate::R<REGION_ATTRIBUTE3rs>;
///Register `REGION_ATTRIBUTE3` writer
pub type W = crate::W<REGION_ATTRIBUTE3rs>;
///Field `FILTER_EN` reader - FILTER_EN
pub type FILTER_EN_R = crate::FieldReader;
///Field `FILTER_EN` writer - FILTER_EN
pub type FILTER_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `S_RD_EN` reader - S_RD_EN
pub type S_RD_EN_R = crate::BitReader;
///Field `S_RD_EN` writer - S_RD_EN
pub type S_RD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `S_WR_EN` reader - S_WR_EN
pub type S_WR_EN_R = crate::BitReader;
///Field `S_WR_EN` writer - S_WR_EN
pub type S_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - FILTER_EN
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new((self.bits & 3) as u8)
    }
    ///Bit 30 - S_RD_EN
    #[inline(always)]
    pub fn s_rd_en(&self) -> S_RD_EN_R {
        S_RD_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - S_WR_EN
    #[inline(always)]
    pub fn s_wr_en(&self) -> S_WR_EN_R {
        S_WR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_ATTRIBUTE3")
            .field("filter_en", &self.filter_en())
            .field("s_rd_en", &self.s_rd_en())
            .field("s_wr_en", &self.s_wr_en())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - FILTER_EN
    #[inline(always)]
    pub fn filter_en(&mut self) -> FILTER_EN_W<'_, REGION_ATTRIBUTE3rs> {
        FILTER_EN_W::new(self, 0)
    }
    ///Bit 30 - S_RD_EN
    #[inline(always)]
    pub fn s_rd_en(&mut self) -> S_RD_EN_W<'_, REGION_ATTRIBUTE3rs> {
        S_RD_EN_W::new(self, 30)
    }
    ///Bit 31 - S_WR_EN
    #[inline(always)]
    pub fn s_wr_en(&mut self) -> S_WR_EN_W<'_, REGION_ATTRIBUTE3rs> {
        S_WR_EN_W::new(self, 31)
    }
}
/**Region x attributes.

You can [`read`](crate::Reg::read) this register and get [`region_attribute3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attribute3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:REGION_ATTRIBUTE3)*/
pub struct REGION_ATTRIBUTE3rs;
impl crate::RegisterSpec for REGION_ATTRIBUTE3rs {
    type Ux = u32;
}
///`read()` method returns [`region_attribute3::R`](R) reader structure
impl crate::Readable for REGION_ATTRIBUTE3rs {}
///`write(|w| ..)` method takes [`region_attribute3::W`](W) writer structure
impl crate::Writable for REGION_ATTRIBUTE3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REGION_ATTRIBUTE3 to value 0
impl crate::Resettable for REGION_ATTRIBUTE3rs {}
