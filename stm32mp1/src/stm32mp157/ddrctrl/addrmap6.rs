///Register `ADDRMAP6` reader
pub type R = crate::R<ADDRMAP6rs>;
///Register `ADDRMAP6` writer
pub type W = crate::W<ADDRMAP6rs>;
///Field `ADDRMAP_ROW_B12` reader - ADDRMAP_ROW_B12
pub type ADDRMAP_ROW_B12_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B12` writer - ADDRMAP_ROW_B12
pub type ADDRMAP_ROW_B12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_ROW_B13` reader - ADDRMAP_ROW_B13
pub type ADDRMAP_ROW_B13_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B13` writer - ADDRMAP_ROW_B13
pub type ADDRMAP_ROW_B13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_ROW_B14` reader - ADDRMAP_ROW_B14
pub type ADDRMAP_ROW_B14_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B14` writer - ADDRMAP_ROW_B14
pub type ADDRMAP_ROW_B14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_ROW_B15` reader - ADDRMAP_ROW_B15
pub type ADDRMAP_ROW_B15_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B15` writer - ADDRMAP_ROW_B15
pub type ADDRMAP_ROW_B15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LPDDR3_6GB_12GB` reader - LPDDR3_6GB_12GB
pub type LPDDR3_6GB_12GB_R = crate::BitReader;
///Field `LPDDR3_6GB_12GB` writer - LPDDR3_6GB_12GB
pub type LPDDR3_6GB_12GB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - ADDRMAP_ROW_B12
    #[inline(always)]
    pub fn addrmap_row_b12(&self) -> ADDRMAP_ROW_B12_R {
        ADDRMAP_ROW_B12_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - ADDRMAP_ROW_B13
    #[inline(always)]
    pub fn addrmap_row_b13(&self) -> ADDRMAP_ROW_B13_R {
        ADDRMAP_ROW_B13_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - ADDRMAP_ROW_B14
    #[inline(always)]
    pub fn addrmap_row_b14(&self) -> ADDRMAP_ROW_B14_R {
        ADDRMAP_ROW_B14_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - ADDRMAP_ROW_B15
    #[inline(always)]
    pub fn addrmap_row_b15(&self) -> ADDRMAP_ROW_B15_R {
        ADDRMAP_ROW_B15_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 31 - LPDDR3_6GB_12GB
    #[inline(always)]
    pub fn lpddr3_6gb_12gb(&self) -> LPDDR3_6GB_12GB_R {
        LPDDR3_6GB_12GB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRMAP6")
            .field("addrmap_row_b12", &self.addrmap_row_b12())
            .field("addrmap_row_b13", &self.addrmap_row_b13())
            .field("addrmap_row_b14", &self.addrmap_row_b14())
            .field("addrmap_row_b15", &self.addrmap_row_b15())
            .field("lpddr3_6gb_12gb", &self.lpddr3_6gb_12gb())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ADDRMAP_ROW_B12
    #[inline(always)]
    pub fn addrmap_row_b12(&mut self) -> ADDRMAP_ROW_B12_W<'_, ADDRMAP6rs> {
        ADDRMAP_ROW_B12_W::new(self, 0)
    }
    ///Bits 8:11 - ADDRMAP_ROW_B13
    #[inline(always)]
    pub fn addrmap_row_b13(&mut self) -> ADDRMAP_ROW_B13_W<'_, ADDRMAP6rs> {
        ADDRMAP_ROW_B13_W::new(self, 8)
    }
    ///Bits 16:19 - ADDRMAP_ROW_B14
    #[inline(always)]
    pub fn addrmap_row_b14(&mut self) -> ADDRMAP_ROW_B14_W<'_, ADDRMAP6rs> {
        ADDRMAP_ROW_B14_W::new(self, 16)
    }
    ///Bits 24:27 - ADDRMAP_ROW_B15
    #[inline(always)]
    pub fn addrmap_row_b15(&mut self) -> ADDRMAP_ROW_B15_W<'_, ADDRMAP6rs> {
        ADDRMAP_ROW_B15_W::new(self, 24)
    }
    ///Bit 31 - LPDDR3_6GB_12GB
    #[inline(always)]
    pub fn lpddr3_6gb_12gb(&mut self) -> LPDDR3_6GB_12GB_W<'_, ADDRMAP6rs> {
        LPDDR3_6GB_12GB_W::new(self, 31)
    }
}
/**DDRCTRL address register 6

You can [`read`](crate::Reg::read) this register and get [`addrmap6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP6)*/
pub struct ADDRMAP6rs;
impl crate::RegisterSpec for ADDRMAP6rs {
    type Ux = u32;
}
///`read()` method returns [`addrmap6::R`](R) reader structure
impl crate::Readable for ADDRMAP6rs {}
///`write(|w| ..)` method takes [`addrmap6::W`](W) writer structure
impl crate::Writable for ADDRMAP6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDRMAP6 to value 0
impl crate::Resettable for ADDRMAP6rs {}
