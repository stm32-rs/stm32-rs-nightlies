///Register `ADDRMAP5` reader
pub type R = crate::R<ADDRMAP5rs>;
///Register `ADDRMAP5` writer
pub type W = crate::W<ADDRMAP5rs>;
///Field `ADDRMAP_ROW_B0` reader - ADDRMAP_ROW_B0
pub type ADDRMAP_ROW_B0_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B0` writer - ADDRMAP_ROW_B0
pub type ADDRMAP_ROW_B0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_ROW_B1` reader - ADDRMAP_ROW_B1
pub type ADDRMAP_ROW_B1_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B1` writer - ADDRMAP_ROW_B1
pub type ADDRMAP_ROW_B1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_ROW_B2_10` reader - ADDRMAP_ROW_B2_10
pub type ADDRMAP_ROW_B2_10_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B2_10` writer - ADDRMAP_ROW_B2_10
pub type ADDRMAP_ROW_B2_10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_ROW_B11` reader - ADDRMAP_ROW_B11
pub type ADDRMAP_ROW_B11_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B11` writer - ADDRMAP_ROW_B11
pub type ADDRMAP_ROW_B11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - ADDRMAP_ROW_B0
    #[inline(always)]
    pub fn addrmap_row_b0(&self) -> ADDRMAP_ROW_B0_R {
        ADDRMAP_ROW_B0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - ADDRMAP_ROW_B1
    #[inline(always)]
    pub fn addrmap_row_b1(&self) -> ADDRMAP_ROW_B1_R {
        ADDRMAP_ROW_B1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - ADDRMAP_ROW_B2_10
    #[inline(always)]
    pub fn addrmap_row_b2_10(&self) -> ADDRMAP_ROW_B2_10_R {
        ADDRMAP_ROW_B2_10_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - ADDRMAP_ROW_B11
    #[inline(always)]
    pub fn addrmap_row_b11(&self) -> ADDRMAP_ROW_B11_R {
        ADDRMAP_ROW_B11_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRMAP5")
            .field("addrmap_row_b0", &self.addrmap_row_b0())
            .field("addrmap_row_b1", &self.addrmap_row_b1())
            .field("addrmap_row_b2_10", &self.addrmap_row_b2_10())
            .field("addrmap_row_b11", &self.addrmap_row_b11())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ADDRMAP_ROW_B0
    #[inline(always)]
    pub fn addrmap_row_b0(&mut self) -> ADDRMAP_ROW_B0_W<'_, ADDRMAP5rs> {
        ADDRMAP_ROW_B0_W::new(self, 0)
    }
    ///Bits 8:11 - ADDRMAP_ROW_B1
    #[inline(always)]
    pub fn addrmap_row_b1(&mut self) -> ADDRMAP_ROW_B1_W<'_, ADDRMAP5rs> {
        ADDRMAP_ROW_B1_W::new(self, 8)
    }
    ///Bits 16:19 - ADDRMAP_ROW_B2_10
    #[inline(always)]
    pub fn addrmap_row_b2_10(&mut self) -> ADDRMAP_ROW_B2_10_W<'_, ADDRMAP5rs> {
        ADDRMAP_ROW_B2_10_W::new(self, 16)
    }
    ///Bits 24:27 - ADDRMAP_ROW_B11
    #[inline(always)]
    pub fn addrmap_row_b11(&mut self) -> ADDRMAP_ROW_B11_W<'_, ADDRMAP5rs> {
        ADDRMAP_ROW_B11_W::new(self, 24)
    }
}
/**DDRCTRL address map register 5

You can [`read`](crate::Reg::read) this register and get [`addrmap5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP5)*/
pub struct ADDRMAP5rs;
impl crate::RegisterSpec for ADDRMAP5rs {
    type Ux = u32;
}
///`read()` method returns [`addrmap5::R`](R) reader structure
impl crate::Readable for ADDRMAP5rs {}
///`write(|w| ..)` method takes [`addrmap5::W`](W) writer structure
impl crate::Writable for ADDRMAP5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDRMAP5 to value 0
impl crate::Resettable for ADDRMAP5rs {}
