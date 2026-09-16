///Register `MEMRMP` reader
pub type R = crate::R<MEMRMPrs>;
///Register `MEMRMP` writer
pub type W = crate::W<MEMRMPrs>;
/**Memory mapping selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM_BOOT {
    ///0: Boot memory base address is defined by FLASH.OPTCR1.BOOT_ADD0
    Add0 = 0,
    ///1: Boot memory base address is defined by FLASH.OPTCR1.BOOT_ADD1
    Add1 = 1,
}
impl From<MEM_BOOT> for bool {
    #[inline(always)]
    fn from(variant: MEM_BOOT) -> Self {
        variant as u8 != 0
    }
}
///Field `MEM_BOOT` reader - Memory mapping selection
pub type MEM_BOOT_R = crate::BitReader<MEM_BOOT>;
impl MEM_BOOT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MEM_BOOT {
        match self.bits {
            false => MEM_BOOT::Add0,
            true => MEM_BOOT::Add1,
        }
    }
    ///Boot memory base address is defined by FLASH.OPTCR1.BOOT_ADD0
    #[inline(always)]
    pub fn is_add0(&self) -> bool {
        *self == MEM_BOOT::Add0
    }
    ///Boot memory base address is defined by FLASH.OPTCR1.BOOT_ADD1
    #[inline(always)]
    pub fn is_add1(&self) -> bool {
        *self == MEM_BOOT::Add1
    }
}
///Field `MEM_BOOT` writer - Memory mapping selection
pub type MEM_BOOT_W<'a, REG> = crate::BitWriter<'a, REG, MEM_BOOT>;
impl<'a, REG> MEM_BOOT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Boot memory base address is defined by FLASH.OPTCR1.BOOT_ADD0
    #[inline(always)]
    pub fn add0(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_BOOT::Add0)
    }
    ///Boot memory base address is defined by FLASH.OPTCR1.BOOT_ADD1
    #[inline(always)]
    pub fn add1(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_BOOT::Add1)
    }
}
/**FMC memory mapping swap

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWP_FMC {
    ///0: No FMC memory mapping swap
    NoSwap = 0,
    ///1: SDRAM banks and NOR/RAM mapping are swapped
    Swapped = 1,
}
impl From<SWP_FMC> for u8 {
    #[inline(always)]
    fn from(variant: SWP_FMC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWP_FMC {
    type Ux = u8;
}
impl crate::IsEnum for SWP_FMC {}
///Field `SWP_FMC` reader - FMC memory mapping swap
pub type SWP_FMC_R = crate::FieldReader<SWP_FMC>;
impl SWP_FMC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWP_FMC> {
        match self.bits {
            0 => Some(SWP_FMC::NoSwap),
            1 => Some(SWP_FMC::Swapped),
            _ => None,
        }
    }
    ///No FMC memory mapping swap
    #[inline(always)]
    pub fn is_no_swap(&self) -> bool {
        *self == SWP_FMC::NoSwap
    }
    ///SDRAM banks and NOR/RAM mapping are swapped
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == SWP_FMC::Swapped
    }
}
///Field `SWP_FMC` writer - FMC memory mapping swap
pub type SWP_FMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SWP_FMC>;
impl<'a, REG> SWP_FMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No FMC memory mapping swap
    #[inline(always)]
    pub fn no_swap(self) -> &'a mut crate::W<REG> {
        self.variant(SWP_FMC::NoSwap)
    }
    ///SDRAM banks and NOR/RAM mapping are swapped
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(SWP_FMC::Swapped)
    }
}
impl R {
    ///Bit 0 - Memory mapping selection
    #[inline(always)]
    pub fn mem_boot(&self) -> MEM_BOOT_R {
        MEM_BOOT_R::new((self.bits & 1) != 0)
    }
    ///Bits 10:11 - FMC memory mapping swap
    #[inline(always)]
    pub fn swp_fmc(&self) -> SWP_FMC_R {
        SWP_FMC_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMRMP")
            .field("mem_boot", &self.mem_boot())
            .field("swp_fmc", &self.swp_fmc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Memory mapping selection
    #[inline(always)]
    pub fn mem_boot(&mut self) -> MEM_BOOT_W<'_, MEMRMPrs> {
        MEM_BOOT_W::new(self, 0)
    }
    ///Bits 10:11 - FMC memory mapping swap
    #[inline(always)]
    pub fn swp_fmc(&mut self) -> SWP_FMC_W<'_, MEMRMPrs> {
        SWP_FMC_W::new(self, 10)
    }
}
/**memory remap register

You can [`read`](crate::Reg::read) this register and get [`memrmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#SYSCFG:MEMRMP)*/
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
