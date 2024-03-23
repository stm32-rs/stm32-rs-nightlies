#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1rs>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1rs>;
#[doc = "Memory mapping selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE {
    #[doc = "0: Main Flash memory mapped at 0x0000_0000"]
    MainFlash = 0,
    #[doc = "1: System Flash memory mapped at 0x0000_0000"]
    SystemFlash = 1,
    #[doc = "3: Embedded SRAM mapped at 0x0000_0000"]
    Sram = 3,
}
impl From<MEM_MODE> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MEM_MODE {
    type Ux = u8;
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MEM_MODE_R = crate::FieldReader<MEM_MODE>;
impl MEM_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MEM_MODE> {
        match self.bits {
            0 => Some(MEM_MODE::MainFlash),
            1 => Some(MEM_MODE::SystemFlash),
            3 => Some(MEM_MODE::Sram),
            _ => None,
        }
    }
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE::MainFlash
    }
    #[doc = "System Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE::SystemFlash
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE::Sram
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MEM_MODE>;
impl<'a, REG> MEM_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::MainFlash)
    }
    #[doc = "System Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::SystemFlash)
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Sram)
    }
}
#[doc = "Boot mode selected by the boot pins status bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOT_MODE {
    #[doc = "0: Main Flash memory boot mode"]
    MainFlash = 0,
    #[doc = "1: System Flash memory boot mode"]
    SystemFlash = 1,
    #[doc = "3: Embedded SRAM boot mode"]
    Sram = 3,
}
impl From<BOOT_MODE> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOOT_MODE {
    type Ux = u8;
}
#[doc = "Field `BOOT_MODE` reader - Boot mode selected by the boot pins status bits"]
pub type BOOT_MODE_R = crate::FieldReader<BOOT_MODE>;
impl BOOT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BOOT_MODE> {
        match self.bits {
            0 => Some(BOOT_MODE::MainFlash),
            1 => Some(BOOT_MODE::SystemFlash),
            3 => Some(BOOT_MODE::Sram),
            _ => None,
        }
    }
    #[doc = "Main Flash memory boot mode"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == BOOT_MODE::MainFlash
    }
    #[doc = "System Flash memory boot mode"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == BOOT_MODE::SystemFlash
    }
    #[doc = "Embedded SRAM boot mode"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == BOOT_MODE::Sram
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Boot mode selected by the boot pins status bits"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
}
#[doc = "SYSCFG configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
