///Register `HWCFGR8` reader
pub type R = crate::R<HWCFGR8rs>;
///Field `AF_PRIO8` reader - AF_PRIO8
pub type AF_PRIO8_R = crate::FieldReader;
///Field `AF_PRIO9` reader - AF_PRIO9
pub type AF_PRIO9_R = crate::FieldReader;
///Field `AF_PRIO10` reader - AF_PRIO10
pub type AF_PRIO10_R = crate::FieldReader;
///Field `AF_PRIO11` reader - AF_PRIO11
pub type AF_PRIO11_R = crate::FieldReader;
///Field `AF_PRIO12` reader - AF_PRIO12
pub type AF_PRIO12_R = crate::FieldReader;
///Field `AF_PRIO13` reader - AF_PRIO13
pub type AF_PRIO13_R = crate::FieldReader;
///Field `AF_PRIO14` reader - AF_PRIO14
pub type AF_PRIO14_R = crate::FieldReader;
///Field `AF_PRIO15` reader - AF_PRIO15
pub type AF_PRIO15_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - AF_PRIO8
    #[inline(always)]
    pub fn af_prio8(&self) -> AF_PRIO8_R {
        AF_PRIO8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - AF_PRIO9
    #[inline(always)]
    pub fn af_prio9(&self) -> AF_PRIO9_R {
        AF_PRIO9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - AF_PRIO10
    #[inline(always)]
    pub fn af_prio10(&self) -> AF_PRIO10_R {
        AF_PRIO10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - AF_PRIO11
    #[inline(always)]
    pub fn af_prio11(&self) -> AF_PRIO11_R {
        AF_PRIO11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - AF_PRIO12
    #[inline(always)]
    pub fn af_prio12(&self) -> AF_PRIO12_R {
        AF_PRIO12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - AF_PRIO13
    #[inline(always)]
    pub fn af_prio13(&self) -> AF_PRIO13_R {
        AF_PRIO13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - AF_PRIO14
    #[inline(always)]
    pub fn af_prio14(&self) -> AF_PRIO14_R {
        AF_PRIO14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - AF_PRIO15
    #[inline(always)]
    pub fn af_prio15(&self) -> AF_PRIO15_R {
        AF_PRIO15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR8")
            .field("af_prio8", &self.af_prio8())
            .field("af_prio9", &self.af_prio9())
            .field("af_prio10", &self.af_prio10())
            .field("af_prio11", &self.af_prio11())
            .field("af_prio12", &self.af_prio12())
            .field("af_prio13", &self.af_prio13())
            .field("af_prio14", &self.af_prio14())
            .field("af_prio15", &self.af_prio15())
            .finish()
    }
}
/**For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`hwcfgr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOE:HWCFGR8)*/
pub struct HWCFGR8rs;
impl crate::RegisterSpec for HWCFGR8rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr8::R`](R) reader structure
impl crate::Readable for HWCFGR8rs {}
///`reset()` method sets HWCFGR8 to value 0
impl crate::Resettable for HWCFGR8rs {}
