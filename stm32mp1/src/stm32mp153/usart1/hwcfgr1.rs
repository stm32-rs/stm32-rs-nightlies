///Register `HWCFGR1` reader
pub type R = crate::R<HWCFGR1rs>;
///Field `CFG1` reader - CFG1
pub type CFG1_R = crate::FieldReader;
///Field `CFG2` reader - CFG2
pub type CFG2_R = crate::FieldReader;
///Field `CFG3` reader - CFG3
pub type CFG3_R = crate::FieldReader;
///Field `CFG4` reader - CFG4
pub type CFG4_R = crate::FieldReader;
///Field `CFG5` reader - CFG5
pub type CFG5_R = crate::FieldReader;
///Field `CFG6` reader - CFG6
pub type CFG6_R = crate::FieldReader;
///Field `CFG7` reader - CFG7
pub type CFG7_R = crate::FieldReader;
///Field `CFG8` reader - CFG8
pub type CFG8_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - CFG1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - CFG2
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - CFG3
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - CFG4
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - CFG5
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - CFG6
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - CFG7
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - CFG8
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR1")
            .field("cfg1", &self.cfg1())
            .field("cfg2", &self.cfg2())
            .field("cfg3", &self.cfg3())
            .field("cfg4", &self.cfg4())
            .field("cfg5", &self.cfg5())
            .field("cfg6", &self.cfg6())
            .field("cfg7", &self.cfg7())
            .field("cfg8", &self.cfg8())
            .finish()
    }
}
/**USART Hardware Configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#USART1:HWCFGR1)*/
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr1::R`](R) reader structure
impl crate::Readable for HWCFGR1rs {}
///`reset()` method sets HWCFGR1 to value 0x14
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x14;
}
