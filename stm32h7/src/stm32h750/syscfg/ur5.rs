///Register `UR5` reader
pub type R = crate::R<UR5rs>;
///Field `MESAD_1` reader - Mass erase secured area disabled for bank 1
pub type MESAD_1_R = crate::BitReader;
///Field `WRPN_1` reader - Write protection for flash bank 1
pub type WRPN_1_R = crate::FieldReader;
impl R {
    ///Bit 0 - Mass erase secured area disabled for bank 1
    #[inline(always)]
    pub fn mesad_1(&self) -> MESAD_1_R {
        MESAD_1_R::new((self.bits & 1) != 0)
    }
    ///Bits 16:23 - Write protection for flash bank 1
    #[inline(always)]
    pub fn wrpn_1(&self) -> WRPN_1_R {
        WRPN_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR5")
            .field("mesad_1", &self.mesad_1())
            .field("wrpn_1", &self.wrpn_1())
            .finish()
    }
}
/**SYSCFG user register 5

You can [`read`](crate::Reg::read) this register and get [`ur5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#SYSCFG:UR5)*/
pub struct UR5rs;
impl crate::RegisterSpec for UR5rs {
    type Ux = u32;
}
///`read()` method returns [`ur5::R`](R) reader structure
impl crate::Readable for UR5rs {}
///`reset()` method sets UR5 to value 0
impl crate::Resettable for UR5rs {}
