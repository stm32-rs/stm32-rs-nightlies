///Register `DDRCTRL_ADDRMAP10` reader
pub type R = crate::R<DDRCTRL_ADDRMAP10rs>;
///Register `DDRCTRL_ADDRMAP10` writer
pub type W = crate::W<DDRCTRL_ADDRMAP10rs>;
///Field `ADDRMAP_ROW_B6` reader - ADDRMAP_ROW_B6
pub type ADDRMAP_ROW_B6_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B6` writer - ADDRMAP_ROW_B6
pub type ADDRMAP_ROW_B6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_ROW_B7` reader - ADDRMAP_ROW_B7
pub type ADDRMAP_ROW_B7_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B7` writer - ADDRMAP_ROW_B7
pub type ADDRMAP_ROW_B7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_ROW_B8` reader - ADDRMAP_ROW_B8
pub type ADDRMAP_ROW_B8_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B8` writer - ADDRMAP_ROW_B8
pub type ADDRMAP_ROW_B8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ADDRMAP_ROW_B9` reader - ADDRMAP_ROW_B9
pub type ADDRMAP_ROW_B9_R = crate::FieldReader;
///Field `ADDRMAP_ROW_B9` writer - ADDRMAP_ROW_B9
pub type ADDRMAP_ROW_B9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - ADDRMAP_ROW_B6
    #[inline(always)]
    pub fn addrmap_row_b6(&self) -> ADDRMAP_ROW_B6_R {
        ADDRMAP_ROW_B6_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - ADDRMAP_ROW_B7
    #[inline(always)]
    pub fn addrmap_row_b7(&self) -> ADDRMAP_ROW_B7_R {
        ADDRMAP_ROW_B7_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - ADDRMAP_ROW_B8
    #[inline(always)]
    pub fn addrmap_row_b8(&self) -> ADDRMAP_ROW_B8_R {
        ADDRMAP_ROW_B8_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - ADDRMAP_ROW_B9
    #[inline(always)]
    pub fn addrmap_row_b9(&self) -> ADDRMAP_ROW_B9_R {
        ADDRMAP_ROW_B9_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_ADDRMAP10")
            .field("addrmap_row_b6", &self.addrmap_row_b6())
            .field("addrmap_row_b7", &self.addrmap_row_b7())
            .field("addrmap_row_b8", &self.addrmap_row_b8())
            .field("addrmap_row_b9", &self.addrmap_row_b9())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ADDRMAP_ROW_B6
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b6(&mut self) -> ADDRMAP_ROW_B6_W<DDRCTRL_ADDRMAP10rs> {
        ADDRMAP_ROW_B6_W::new(self, 0)
    }
    ///Bits 8:11 - ADDRMAP_ROW_B7
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b7(&mut self) -> ADDRMAP_ROW_B7_W<DDRCTRL_ADDRMAP10rs> {
        ADDRMAP_ROW_B7_W::new(self, 8)
    }
    ///Bits 16:19 - ADDRMAP_ROW_B8
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b8(&mut self) -> ADDRMAP_ROW_B8_W<DDRCTRL_ADDRMAP10rs> {
        ADDRMAP_ROW_B8_W::new(self, 16)
    }
    ///Bits 24:27 - ADDRMAP_ROW_B9
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b9(&mut self) -> ADDRMAP_ROW_B9_W<DDRCTRL_ADDRMAP10rs> {
        ADDRMAP_ROW_B9_W::new(self, 24)
    }
}
/**DDRCTRL address map register 10

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_addrmap10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_addrmap10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DDRCTRL_ADDRMAP10)*/
pub struct DDRCTRL_ADDRMAP10rs;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP10rs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_addrmap10::R`](R) reader structure
impl crate::Readable for DDRCTRL_ADDRMAP10rs {}
///`write(|w| ..)` method takes [`ddrctrl_addrmap10::W`](W) writer structure
impl crate::Writable for DDRCTRL_ADDRMAP10rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_ADDRMAP10 to value 0
impl crate::Resettable for DDRCTRL_ADDRMAP10rs {
    const RESET_VALUE: u32 = 0;
}
