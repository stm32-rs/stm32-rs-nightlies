#[doc = "Register `MEMRMP` reader"]
pub type R = crate::R<MEMRMPrs>;
#[doc = "Register `MEMRMP` writer"]
pub type W = crate::W<MEMRMPrs>;
#[doc = "Memory mapping selection\n\nValue on reset: 0"]
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
#[doc = "Field `MEM_MODE` reader - Memory mapping selection"]
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
#[doc = "Field `MEM_MODE` writer - Memory mapping selection"]
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MEM_MODE>;
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
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<MEMRMPrs> {
        MEM_MODE_W::new(self, 0)
    }
}
#[doc = "memory remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memrmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memrmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEMRMPrs;
impl crate::RegisterSpec for MEMRMPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memrmp::R`](R) reader structure"]
impl crate::Readable for MEMRMPrs {}
#[doc = "`write(|w| ..)` method takes [`memrmp::W`](W) writer structure"]
impl crate::Writable for MEMRMPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMRMP to value 0"]
impl crate::Resettable for MEMRMPrs {
    const RESET_VALUE: u32 = 0;
}
