///Register `ADDRMAP2` reader
pub type R = crate::R<ADDRMAP2rs>;
///Register `ADDRMAP2` writer
pub type W = crate::W<ADDRMAP2rs>;
///Field `ADDRMAP_COL_B2` reader - ADDRMAP_COL_B2
pub type ADDRMAP_COL_B2_R = crate::FieldReader;
///Field `ADDRMAP_COL_B2` writer - ADDRMAP_COL_B2
pub type ADDRMAP_COL_B2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_COL_B3` reader - ADDRMAP_COL_B3
pub type ADDRMAP_COL_B3_R = crate::FieldReader;
///Field `ADDRMAP_COL_B3` writer - ADDRMAP_COL_B3
pub type ADDRMAP_COL_B3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_COL_B4` reader - ADDRMAP_COL_B4
pub type ADDRMAP_COL_B4_R = crate::FieldReader;
///Field `ADDRMAP_COL_B4` writer - ADDRMAP_COL_B4
pub type ADDRMAP_COL_B4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_COL_B5` reader - ADDRMAP_COL_B5
pub type ADDRMAP_COL_B5_R = crate::FieldReader;
///Field `ADDRMAP_COL_B5` writer - ADDRMAP_COL_B5
pub type ADDRMAP_COL_B5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - ADDRMAP_COL_B2
    #[inline(always)]
    pub fn addrmap_col_b2(&self) -> ADDRMAP_COL_B2_R {
        ADDRMAP_COL_B2_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - ADDRMAP_COL_B3
    #[inline(always)]
    pub fn addrmap_col_b3(&self) -> ADDRMAP_COL_B3_R {
        ADDRMAP_COL_B3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - ADDRMAP_COL_B4
    #[inline(always)]
    pub fn addrmap_col_b4(&self) -> ADDRMAP_COL_B4_R {
        ADDRMAP_COL_B4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - ADDRMAP_COL_B5
    #[inline(always)]
    pub fn addrmap_col_b5(&self) -> ADDRMAP_COL_B5_R {
        ADDRMAP_COL_B5_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRMAP2")
            .field("addrmap_col_b2", &self.addrmap_col_b2())
            .field("addrmap_col_b3", &self.addrmap_col_b3())
            .field("addrmap_col_b4", &self.addrmap_col_b4())
            .field("addrmap_col_b5", &self.addrmap_col_b5())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ADDRMAP_COL_B2
    #[inline(always)]
    pub fn addrmap_col_b2(&mut self) -> ADDRMAP_COL_B2_W<'_, ADDRMAP2rs> {
        ADDRMAP_COL_B2_W::new(self, 0)
    }
    ///Bits 8:11 - ADDRMAP_COL_B3
    #[inline(always)]
    pub fn addrmap_col_b3(&mut self) -> ADDRMAP_COL_B3_W<'_, ADDRMAP2rs> {
        ADDRMAP_COL_B3_W::new(self, 8)
    }
    ///Bits 16:19 - ADDRMAP_COL_B4
    #[inline(always)]
    pub fn addrmap_col_b4(&mut self) -> ADDRMAP_COL_B4_W<'_, ADDRMAP2rs> {
        ADDRMAP_COL_B4_W::new(self, 16)
    }
    ///Bits 24:27 - ADDRMAP_COL_B5
    #[inline(always)]
    pub fn addrmap_col_b5(&mut self) -> ADDRMAP_COL_B5_W<'_, ADDRMAP2rs> {
        ADDRMAP_COL_B5_W::new(self, 24)
    }
}
/**DDRCTRL address map register 2

You can [`read`](crate::Reg::read) this register and get [`addrmap2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP2)*/
pub struct ADDRMAP2rs;
impl crate::RegisterSpec for ADDRMAP2rs {
    type Ux = u32;
}
///`read()` method returns [`addrmap2::R`](R) reader structure
impl crate::Readable for ADDRMAP2rs {}
///`write(|w| ..)` method takes [`addrmap2::W`](W) writer structure
impl crate::Writable for ADDRMAP2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDRMAP2 to value 0
impl crate::Resettable for ADDRMAP2rs {}
