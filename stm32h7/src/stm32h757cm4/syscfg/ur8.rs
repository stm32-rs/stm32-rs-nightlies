///Register `UR8` reader
pub type R = crate::R<UR8rs>;
///Field `MEPAD_2` reader - Mass erase protected area disabled for bank 2
pub type MEPAD_2_R = crate::BitReader;
///Field `MESAD_2` reader - Mass erase secured area disabled for bank 2
pub type MESAD_2_R = crate::BitReader;
impl R {
    ///Bit 0 - Mass erase protected area disabled for bank 2
    #[inline(always)]
    pub fn mepad_2(&self) -> MEPAD_2_R {
        MEPAD_2_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - Mass erase secured area disabled for bank 2
    #[inline(always)]
    pub fn mesad_2(&self) -> MESAD_2_R {
        MESAD_2_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR8")
            .field("mepad_2", &self.mepad_2())
            .field("mesad_2", &self.mesad_2())
            .finish()
    }
}
/**SYSCFG user register 8

You can [`read`](crate::Reg::read) this register and get [`ur8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#SYSCFG:UR8)*/
pub struct UR8rs;
impl crate::RegisterSpec for UR8rs {
    type Ux = u32;
}
///`read()` method returns [`ur8::R`](R) reader structure
impl crate::Readable for UR8rs {}
///`reset()` method sets UR8 to value 0
impl crate::Resettable for UR8rs {}
