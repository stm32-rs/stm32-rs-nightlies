///Register `ADDRMAP4` reader
pub type R = crate::R<ADDRMAP4rs>;
///Register `ADDRMAP4` writer
pub type W = crate::W<ADDRMAP4rs>;
///Field `ADDRMAP_COL_B10` reader - ADDRMAP_COL_B10
pub type ADDRMAP_COL_B10_R = crate::FieldReader;
///Field `ADDRMAP_COL_B10` writer - ADDRMAP_COL_B10
pub type ADDRMAP_COL_B10_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `ADDRMAP_COL_B11` reader - ADDRMAP_COL_B11
pub type ADDRMAP_COL_B11_R = crate::FieldReader;
///Field `ADDRMAP_COL_B11` writer - ADDRMAP_COL_B11
pub type ADDRMAP_COL_B11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - ADDRMAP_COL_B10
    #[inline(always)]
    pub fn addrmap_col_b10(&self) -> ADDRMAP_COL_B10_R {
        ADDRMAP_COL_B10_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - ADDRMAP_COL_B11
    #[inline(always)]
    pub fn addrmap_col_b11(&self) -> ADDRMAP_COL_B11_R {
        ADDRMAP_COL_B11_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRMAP4")
            .field("addrmap_col_b10", &self.addrmap_col_b10())
            .field("addrmap_col_b11", &self.addrmap_col_b11())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - ADDRMAP_COL_B10
    #[inline(always)]
    pub fn addrmap_col_b10(&mut self) -> ADDRMAP_COL_B10_W<'_, ADDRMAP4rs> {
        ADDRMAP_COL_B10_W::new(self, 0)
    }
    ///Bits 8:12 - ADDRMAP_COL_B11
    #[inline(always)]
    pub fn addrmap_col_b11(&mut self) -> ADDRMAP_COL_B11_W<'_, ADDRMAP4rs> {
        ADDRMAP_COL_B11_W::new(self, 8)
    }
}
/**DDRCTRL address map register 4

You can [`read`](crate::Reg::read) this register and get [`addrmap4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:ADDRMAP4)*/
pub struct ADDRMAP4rs;
impl crate::RegisterSpec for ADDRMAP4rs {
    type Ux = u32;
}
///`read()` method returns [`addrmap4::R`](R) reader structure
impl crate::Readable for ADDRMAP4rs {}
///`write(|w| ..)` method takes [`addrmap4::W`](W) writer structure
impl crate::Writable for ADDRMAP4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDRMAP4 to value 0
impl crate::Resettable for ADDRMAP4rs {}
