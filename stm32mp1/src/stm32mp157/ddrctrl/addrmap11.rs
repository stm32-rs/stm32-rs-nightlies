///Register `ADDRMAP11` reader
pub type R = crate::R<ADDRMAP11rs>;
///Register `ADDRMAP11` writer
pub type W = crate::W<ADDRMAP11rs>;
///Field `ADDRMAP_ROW_B10` reader - ADDRMAP_ROW_B10
pub type ADDRMAP_ROW_B10_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B10` writer - ADDRMAP_ROW_B10
pub type ADDRMAP_ROW_B10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - ADDRMAP_ROW_B10
    #[inline(always)]
    pub fn addrmap_row_b10(&self) -> ADDRMAP_ROW_B10_R {
        ADDRMAP_ROW_B10_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRMAP11")
            .field("addrmap_row_b10", &self.addrmap_row_b10())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ADDRMAP_ROW_B10
    #[inline(always)]
    pub fn addrmap_row_b10(&mut self) -> ADDRMAP_ROW_B10_W<ADDRMAP11rs> {
        ADDRMAP_ROW_B10_W::new(self, 0)
    }
}
/**DDRCTRL address map register 11

You can [`read`](crate::Reg::read) this register and get [`addrmap11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrmap11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:ADDRMAP11)*/
pub struct ADDRMAP11rs;
impl crate::RegisterSpec for ADDRMAP11rs {
    type Ux = u32;
}
///`read()` method returns [`addrmap11::R`](R) reader structure
impl crate::Readable for ADDRMAP11rs {}
///`write(|w| ..)` method takes [`addrmap11::W`](W) writer structure
impl crate::Writable for ADDRMAP11rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDRMAP11 to value 0
impl crate::Resettable for ADDRMAP11rs {}
