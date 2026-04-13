///Register `MEMRMP` reader
pub type R = crate::R<MEMRMPrs>;
///Register `MEMRMP` writer
pub type W = crate::W<MEMRMPrs>;
/**MEM_MODE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE {
    ///0: Main Flash memory mapped at 0x0000 0000
    MainFlash = 0,
    ///1: System Flash memory mapped at 0x0000 0000
    SystemFlash = 1,
    ///2: FSMC Bank1 (NOR/PSRAM 1 and 2) mapped at 0x0000 0000
    Fsmcbank1 = 2,
    ///3: Embedded SRAM mapped at 0x0000 0000
    EmbeddedSram = 3,
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
impl crate::IsEnum for MEM_MODE {}
///Field `MEM_MODE` reader - MEM_MODE
pub type MEM_MODE_R = crate::FieldReader<MEM_MODE>;
impl MEM_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MEM_MODE {
        match self.bits {
            0 => MEM_MODE::MainFlash,
            1 => MEM_MODE::SystemFlash,
            2 => MEM_MODE::Fsmcbank1,
            3 => MEM_MODE::EmbeddedSram,
            _ => unreachable!(),
        }
    }
    ///Main Flash memory mapped at 0x0000 0000
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE::MainFlash
    }
    ///System Flash memory mapped at 0x0000 0000
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE::SystemFlash
    }
    ///FSMC Bank1 (NOR/PSRAM 1 and 2) mapped at 0x0000 0000
    #[inline(always)]
    pub fn is_fsmcbank1(&self) -> bool {
        *self == MEM_MODE::Fsmcbank1
    }
    ///Embedded SRAM mapped at 0x0000 0000
    #[inline(always)]
    pub fn is_embedded_sram(&self) -> bool {
        *self == MEM_MODE::EmbeddedSram
    }
}
///Field `MEM_MODE` writer - MEM_MODE
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MEM_MODE, crate::Safe>;
impl<'a, REG> MEM_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Main Flash memory mapped at 0x0000 0000
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::MainFlash)
    }
    ///System Flash memory mapped at 0x0000 0000
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::SystemFlash)
    }
    ///FSMC Bank1 (NOR/PSRAM 1 and 2) mapped at 0x0000 0000
    #[inline(always)]
    pub fn fsmcbank1(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Fsmcbank1)
    }
    ///Embedded SRAM mapped at 0x0000 0000
    #[inline(always)]
    pub fn embedded_sram(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::EmbeddedSram)
    }
}
impl R {
    ///Bits 0:1 - MEM_MODE
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMRMP")
            .field("mem_mode", &self.mem_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - MEM_MODE
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<'_, MEMRMPrs> {
        MEM_MODE_W::new(self, 0)
    }
}
/**memory remap register

You can [`read`](crate::Reg::read) this register and get [`memrmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#SYSCFG:MEMRMP)*/
pub struct MEMRMPrs;
impl crate::RegisterSpec for MEMRMPrs {
    type Ux = u32;
}
///`read()` method returns [`memrmp::R`](R) reader structure
impl crate::Readable for MEMRMPrs {}
///`write(|w| ..)` method takes [`memrmp::W`](W) writer structure
impl crate::Writable for MEMRMPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MEMRMP to value 0
impl crate::Resettable for MEMRMPrs {}
