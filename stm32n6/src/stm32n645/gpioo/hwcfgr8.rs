///Register `HWCFGR8` reader
pub type R = crate::R<HWCFGR8rs>;
///Field `FAST_AF_IO8` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO8_R = crate::FieldReader;
///Field `FAST_AF_IO9` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO9_R = crate::FieldReader;
///Field `FAST_AF_IO10` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO10_R = crate::FieldReader;
///Field `FAST_AF_IO11` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO11_R = crate::FieldReader;
///Field `FAST_AF_IO12` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO12_R = crate::FieldReader;
///Field `FAST_AF_IO13` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO13_R = crate::FieldReader;
///Field `FAST_AF_IO14` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO14_R = crate::FieldReader;
///Field `FAST_AF_IO15` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO15_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io8(&self) -> FAST_AF_IO8_R {
        FAST_AF_IO8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io9(&self) -> FAST_AF_IO9_R {
        FAST_AF_IO9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io10(&self) -> FAST_AF_IO10_R {
        FAST_AF_IO10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io11(&self) -> FAST_AF_IO11_R {
        FAST_AF_IO11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io12(&self) -> FAST_AF_IO12_R {
        FAST_AF_IO12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io13(&self) -> FAST_AF_IO13_R {
        FAST_AF_IO13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io14(&self) -> FAST_AF_IO14_R {
        FAST_AF_IO14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io15(&self) -> FAST_AF_IO15_R {
        FAST_AF_IO15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR8")
            .field("fast_af_io8", &self.fast_af_io8())
            .field("fast_af_io9", &self.fast_af_io9())
            .field("fast_af_io10", &self.fast_af_io10())
            .field("fast_af_io11", &self.fast_af_io11())
            .field("fast_af_io12", &self.fast_af_io12())
            .field("fast_af_io13", &self.fast_af_io13())
            .field("fast_af_io14", &self.fast_af_io14())
            .field("fast_af_io15", &self.fast_af_io15())
            .finish()
    }
}
/**GPIO port O hardware configuration register 8

You can [`read`](crate::Reg::read) this register and get [`hwcfgr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GPIOO:HWCFGR8)*/
pub struct HWCFGR8rs;
impl crate::RegisterSpec for HWCFGR8rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr8::R`](R) reader structure
impl crate::Readable for HWCFGR8rs {}
///`reset()` method sets HWCFGR8 to value 0xffff_ffff
impl crate::Resettable for HWCFGR8rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
