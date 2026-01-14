///Register `MEMRMP` reader
pub type R = crate::R<MEMRMPrs>;
///Register `MEMRMP` writer
pub type W = crate::W<MEMRMPrs>;
/**Memory mapping selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE {
    ///0: Main Flash memory mapped at 0x00000000
    MainFlash = 0,
    ///1: System Flash memory mapped at 0x00000000
    SystemFlash = 1,
    ///2: FMC bank 1 (NOR/PSRAM 1 and 2) mapped at 0x00000000
    Fmc = 2,
    ///3: SRAM1 mapped at 0x00000000
    Sram1 = 3,
    ///4: OCTOSPI1 memory mapped at 0x00000000
    Octospi1 = 4,
    ///5: OCTOSPI2 memory mapped at 0x00000000
    Octospi2 = 5,
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
///Field `MEM_MODE` reader - Memory mapping selection
pub type MEM_MODE_R = crate::FieldReader<MEM_MODE>;
impl MEM_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MEM_MODE> {
        match self.bits {
            0 => Some(MEM_MODE::MainFlash),
            1 => Some(MEM_MODE::SystemFlash),
            2 => Some(MEM_MODE::Fmc),
            3 => Some(MEM_MODE::Sram1),
            4 => Some(MEM_MODE::Octospi1),
            5 => Some(MEM_MODE::Octospi2),
            _ => None,
        }
    }
    ///Main Flash memory mapped at 0x00000000
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE::MainFlash
    }
    ///System Flash memory mapped at 0x00000000
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE::SystemFlash
    }
    ///FMC bank 1 (NOR/PSRAM 1 and 2) mapped at 0x00000000
    #[inline(always)]
    pub fn is_fmc(&self) -> bool {
        *self == MEM_MODE::Fmc
    }
    ///SRAM1 mapped at 0x00000000
    #[inline(always)]
    pub fn is_sram1(&self) -> bool {
        *self == MEM_MODE::Sram1
    }
    ///OCTOSPI1 memory mapped at 0x00000000
    #[inline(always)]
    pub fn is_octospi1(&self) -> bool {
        *self == MEM_MODE::Octospi1
    }
    ///OCTOSPI2 memory mapped at 0x00000000
    #[inline(always)]
    pub fn is_octospi2(&self) -> bool {
        *self == MEM_MODE::Octospi2
    }
}
///Field `MEM_MODE` writer - Memory mapping selection
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MEM_MODE>;
impl<'a, REG> MEM_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Main Flash memory mapped at 0x00000000
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::MainFlash)
    }
    ///System Flash memory mapped at 0x00000000
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::SystemFlash)
    }
    ///FMC bank 1 (NOR/PSRAM 1 and 2) mapped at 0x00000000
    #[inline(always)]
    pub fn fmc(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Fmc)
    }
    ///SRAM1 mapped at 0x00000000
    #[inline(always)]
    pub fn sram1(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Sram1)
    }
    ///OCTOSPI1 memory mapped at 0x00000000
    #[inline(always)]
    pub fn octospi1(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Octospi1)
    }
    ///OCTOSPI2 memory mapped at 0x00000000
    #[inline(always)]
    pub fn octospi2(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Octospi2)
    }
}
/**Flash Bank mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FB_MODE {
    ///0: Flash Bank 1 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 2 mapped at offset
    Normal = 0,
    ///1: Flash Bank 2 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 1 mapped at offset
    Inverted = 1,
}
impl From<FB_MODE> for bool {
    #[inline(always)]
    fn from(variant: FB_MODE) -> Self {
        variant as u8 != 0
    }
}
///Field `FB_MODE` reader - Flash Bank mode selection
pub type FB_MODE_R = crate::BitReader<FB_MODE>;
impl FB_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FB_MODE {
        match self.bits {
            false => FB_MODE::Normal,
            true => FB_MODE::Inverted,
        }
    }
    ///Flash Bank 1 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 2 mapped at offset
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FB_MODE::Normal
    }
    ///Flash Bank 2 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 1 mapped at offset
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == FB_MODE::Inverted
    }
}
///Field `FB_MODE` writer - Flash Bank mode selection
pub type FB_MODE_W<'a, REG> = crate::BitWriter<'a, REG, FB_MODE>;
impl<'a, REG> FB_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash Bank 1 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 2 mapped at offset
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(FB_MODE::Normal)
    }
    ///Flash Bank 2 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 1 mapped at offset
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(FB_MODE::Inverted)
    }
}
impl R {
    ///Bits 0:2 - Memory mapping selection
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - Flash Bank mode selection
    #[inline(always)]
    pub fn fb_mode(&self) -> FB_MODE_R {
        FB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMRMP")
            .field("fb_mode", &self.fb_mode())
            .field("mem_mode", &self.mem_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Memory mapping selection
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<'_, MEMRMPrs> {
        MEM_MODE_W::new(self, 0)
    }
    ///Bit 8 - Flash Bank mode selection
    #[inline(always)]
    pub fn fb_mode(&mut self) -> FB_MODE_W<'_, MEMRMPrs> {
        FB_MODE_W::new(self, 8)
    }
}
/**memory remap register

You can [`read`](crate::Reg::read) this register and get [`memrmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#SYSCFG:MEMRMP)*/
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
