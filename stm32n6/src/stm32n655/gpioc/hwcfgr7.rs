///Register `HWCFGR7` reader
pub type R = crate::R<HWCFGR7rs>;
///Field `FAST_AF_IO0` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO0_R = crate::FieldReader;
///Field `FAST_AF_IO1` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO1_R = crate::FieldReader;
///Field `FAST_AF_IO2` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO2_R = crate::FieldReader;
///Field `FAST_AF_IO3` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO3_R = crate::FieldReader;
///Field `FAST_AF_IO4` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO4_R = crate::FieldReader;
///Field `FAST_AF_IO5` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO5_R = crate::FieldReader;
///Field `FAST_AF_IO6` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO6_R = crate::FieldReader;
///Field `FAST_AF_IO7` reader - Indicate which is the fastest AF for I/Oy (0 to F)
pub type FAST_AF_IO7_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io0(&self) -> FAST_AF_IO0_R {
        FAST_AF_IO0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io1(&self) -> FAST_AF_IO1_R {
        FAST_AF_IO1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io2(&self) -> FAST_AF_IO2_R {
        FAST_AF_IO2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io3(&self) -> FAST_AF_IO3_R {
        FAST_AF_IO3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io4(&self) -> FAST_AF_IO4_R {
        FAST_AF_IO4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io5(&self) -> FAST_AF_IO5_R {
        FAST_AF_IO5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io6(&self) -> FAST_AF_IO6_R {
        FAST_AF_IO6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Indicate which is the fastest AF for I/Oy (0 to F)
    #[inline(always)]
    pub fn fast_af_io7(&self) -> FAST_AF_IO7_R {
        FAST_AF_IO7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR7")
            .field("fast_af_io0", &self.fast_af_io0())
            .field("fast_af_io1", &self.fast_af_io1())
            .field("fast_af_io2", &self.fast_af_io2())
            .field("fast_af_io3", &self.fast_af_io3())
            .field("fast_af_io4", &self.fast_af_io4())
            .field("fast_af_io5", &self.fast_af_io5())
            .field("fast_af_io6", &self.fast_af_io6())
            .field("fast_af_io7", &self.fast_af_io7())
            .finish()
    }
}
/**GPIO port C hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`hwcfgr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GPIOC:HWCFGR7)*/
pub struct HWCFGR7rs;
impl crate::RegisterSpec for HWCFGR7rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr7::R`](R) reader structure
impl crate::Readable for HWCFGR7rs {}
///`reset()` method sets HWCFGR7 to value 0xcccc_cccc
impl crate::Resettable for HWCFGR7rs {
    const RESET_VALUE: u32 = 0xcccc_cccc;
}
