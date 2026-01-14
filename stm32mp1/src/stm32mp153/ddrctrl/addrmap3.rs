///Register `ADDRMAP3` reader
pub type R = crate::R<ADDRMAP3rs>;
///Register `ADDRMAP3` writer
pub type W = crate::W<ADDRMAP3rs>;
///Field `ADDRMAP_COL_B6` reader - ADDRMAP_COL_B6
pub type ADDRMAP_COL_B6_R = crate::FieldReader;
///Field `ADDRMAP_COL_B6` writer - ADDRMAP_COL_B6
pub type ADDRMAP_COL_B6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_COL_B7` reader - ADDRMAP_COL_B7
pub type ADDRMAP_COL_B7_R = crate::FieldReader;
///Field `ADDRMAP_COL_B7` writer - ADDRMAP_COL_B7
pub type ADDRMAP_COL_B7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADDRMAP_COL_B8` reader - ADDRMAP_COL_B8
pub type ADDRMAP_COL_B8_R = crate::FieldReader;
///Field `ADDRMAP_COL_B8` writer - ADDRMAP_COL_B8
pub type ADDRMAP_COL_B8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADDRMAP_COL_B9` reader - ADDRMAP_COL_B9
pub type ADDRMAP_COL_B9_R = crate::FieldReader;
///Field `ADDRMAP_COL_B9` writer - ADDRMAP_COL_B9
pub type ADDRMAP_COL_B9_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:3 - ADDRMAP_COL_B6
    #[inline(always)]
    pub fn addrmap_col_b6(&self) -> ADDRMAP_COL_B6_R {
        ADDRMAP_COL_B6_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:12 - ADDRMAP_COL_B7
    #[inline(always)]
    pub fn addrmap_col_b7(&self) -> ADDRMAP_COL_B7_R {
        ADDRMAP_COL_B7_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - ADDRMAP_COL_B8
    #[inline(always)]
    pub fn addrmap_col_b8(&self) -> ADDRMAP_COL_B8_R {
        ADDRMAP_COL_B8_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28 - ADDRMAP_COL_B9
    #[inline(always)]
    pub fn addrmap_col_b9(&self) -> ADDRMAP_COL_B9_R {
        ADDRMAP_COL_B9_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRMAP3")
            .field("addrmap_col_b6", &self.addrmap_col_b6())
            .field("addrmap_col_b7", &self.addrmap_col_b7())
            .field("addrmap_col_b8", &self.addrmap_col_b8())
            .field("addrmap_col_b9", &self.addrmap_col_b9())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ADDRMAP_COL_B6
    #[inline(always)]
    pub fn addrmap_col_b6(&mut self) -> ADDRMAP_COL_B6_W<'_, ADDRMAP3rs> {
        ADDRMAP_COL_B6_W::new(self, 0)
    }
    ///Bits 8:12 - ADDRMAP_COL_B7
    #[inline(always)]
    pub fn addrmap_col_b7(&mut self) -> ADDRMAP_COL_B7_W<'_, ADDRMAP3rs> {
        ADDRMAP_COL_B7_W::new(self, 8)
    }
    ///Bits 16:20 - ADDRMAP_COL_B8
    #[inline(always)]
    pub fn addrmap_col_b8(&mut self) -> ADDRMAP_COL_B8_W<'_, ADDRMAP3rs> {
        ADDRMAP_COL_B8_W::new(self, 16)
    }
    ///Bits 24:28 - ADDRMAP_COL_B9
    #[inline(always)]
    pub fn addrmap_col_b9(&mut self) -> ADDRMAP_COL_B9_W<'_, ADDRMAP3rs> {
        ADDRMAP_COL_B9_W::new(self, 24)
    }
}
/**DDRCTRL address map register 3

You can [`read`](crate::Reg::read) this register and get [`addrmap3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:ADDRMAP3)*/
pub struct ADDRMAP3rs;
impl crate::RegisterSpec for ADDRMAP3rs {
    type Ux = u32;
}
///`read()` method returns [`addrmap3::R`](R) reader structure
impl crate::Readable for ADDRMAP3rs {}
///`write(|w| ..)` method takes [`addrmap3::W`](W) writer structure
impl crate::Writable for ADDRMAP3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDRMAP3 to value 0
impl crate::Resettable for ADDRMAP3rs {}
