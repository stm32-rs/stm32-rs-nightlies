///Register `DDRCTRL_ADDRMAP11` reader
pub type R = crate::R<DDRCTRL_ADDRMAP11rs>;
///Register `DDRCTRL_ADDRMAP11` writer
pub type W = crate::W<DDRCTRL_ADDRMAP11rs>;
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
        f.debug_struct("DDRCTRL_ADDRMAP11")
            .field("addrmap_row_b10", &self.addrmap_row_b10())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ADDRMAP_ROW_B10
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b10(&mut self) -> ADDRMAP_ROW_B10_W<DDRCTRL_ADDRMAP11rs> {
        ADDRMAP_ROW_B10_W::new(self, 0)
    }
}
/**DDRCTRL address map register 11

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_addrmap11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_addrmap11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DDRCTRL_ADDRMAP11)*/
pub struct DDRCTRL_ADDRMAP11rs;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP11rs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_addrmap11::R`](R) reader structure
impl crate::Readable for DDRCTRL_ADDRMAP11rs {}
///`write(|w| ..)` method takes [`ddrctrl_addrmap11::W`](W) writer structure
impl crate::Writable for DDRCTRL_ADDRMAP11rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_ADDRMAP11 to value 0
impl crate::Resettable for DDRCTRL_ADDRMAP11rs {
    const RESET_VALUE: u32 = 0;
}
