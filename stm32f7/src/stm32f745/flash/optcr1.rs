///Register `OPTCR1` reader
pub type R = crate::R<OPTCR1rs>;
///Register `OPTCR1` writer
pub type W = crate::W<OPTCR1rs>;
/**Boot base address when Boot pin =0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BOOT_ADD0 {
    ///0: Boot from ITCM RAM (0x0000 0000)
    ItcmRam = 0,
    ///64: Boot from System memory bootloader (0x0010 0000)
    SystemMemoryBootloader = 64,
    ///128: Boot from Flash on ITCM interface (0x0020 0000)
    FlashItcmInterface = 128,
    ///8192: Boot from Flash on AXIM interface (0x0800 0000)
    FlashAximInterface = 8192,
    ///32768: Boot from DTCM RAM (0x2000 0000)
    DtcmRam = 32768,
    ///32772: Boot from SRAM1 (0x2001 0000)
    Sram1 = 32772,
    ///32787: Boot from SRAM2 (0x2003 C000)
    Sram2 = 32787,
    ///1: Boot from specified address (granularity of 16KB)
    BootAddress = 1,
}
impl From<BOOT_ADD0> for u16 {
    #[inline(always)]
    fn from(variant: BOOT_ADD0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOOT_ADD0 {
    type Ux = u16;
}
impl crate::IsEnum for BOOT_ADD0 {}
///Field `BOOT_ADD0` reader - Boot base address when Boot pin =0
pub type BOOT_ADD0_R = crate::FieldReader<BOOT_ADD0>;
impl BOOT_ADD0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOOT_ADD0 {
        match self.bits {
            0 => BOOT_ADD0::ItcmRam,
            64 => BOOT_ADD0::SystemMemoryBootloader,
            128 => BOOT_ADD0::FlashItcmInterface,
            8192 => BOOT_ADD0::FlashAximInterface,
            32768 => BOOT_ADD0::DtcmRam,
            32772 => BOOT_ADD0::Sram1,
            32787 => BOOT_ADD0::Sram2,
            _ => BOOT_ADD0::BootAddress,
        }
    }
    ///Boot from ITCM RAM (0x0000 0000)
    #[inline(always)]
    pub fn is_itcm_ram(&self) -> bool {
        *self == BOOT_ADD0::ItcmRam
    }
    ///Boot from System memory bootloader (0x0010 0000)
    #[inline(always)]
    pub fn is_system_memory_bootloader(&self) -> bool {
        *self == BOOT_ADD0::SystemMemoryBootloader
    }
    ///Boot from Flash on ITCM interface (0x0020 0000)
    #[inline(always)]
    pub fn is_flash_itcm_interface(&self) -> bool {
        *self == BOOT_ADD0::FlashItcmInterface
    }
    ///Boot from Flash on AXIM interface (0x0800 0000)
    #[inline(always)]
    pub fn is_flash_axim_interface(&self) -> bool {
        *self == BOOT_ADD0::FlashAximInterface
    }
    ///Boot from DTCM RAM (0x2000 0000)
    #[inline(always)]
    pub fn is_dtcm_ram(&self) -> bool {
        *self == BOOT_ADD0::DtcmRam
    }
    ///Boot from SRAM1 (0x2001 0000)
    #[inline(always)]
    pub fn is_sram1(&self) -> bool {
        *self == BOOT_ADD0::Sram1
    }
    ///Boot from SRAM2 (0x2003 C000)
    #[inline(always)]
    pub fn is_sram2(&self) -> bool {
        *self == BOOT_ADD0::Sram2
    }
    ///Boot from specified address (granularity of 16KB)
    #[inline(always)]
    pub fn is_boot_address(&self) -> bool {
        matches!(self.variant(), BOOT_ADD0::BootAddress)
    }
}
///Field `BOOT_ADD0` writer - Boot base address when Boot pin =0
pub type BOOT_ADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, BOOT_ADD0, crate::Safe>;
impl<'a, REG> BOOT_ADD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Boot from ITCM RAM (0x0000 0000)
    #[inline(always)]
    pub fn itcm_ram(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD0::ItcmRam)
    }
    ///Boot from System memory bootloader (0x0010 0000)
    #[inline(always)]
    pub fn system_memory_bootloader(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD0::SystemMemoryBootloader)
    }
    ///Boot from Flash on ITCM interface (0x0020 0000)
    #[inline(always)]
    pub fn flash_itcm_interface(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD0::FlashItcmInterface)
    }
    ///Boot from Flash on AXIM interface (0x0800 0000)
    #[inline(always)]
    pub fn flash_axim_interface(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD0::FlashAximInterface)
    }
    ///Boot from DTCM RAM (0x2000 0000)
    #[inline(always)]
    pub fn dtcm_ram(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD0::DtcmRam)
    }
    ///Boot from SRAM1 (0x2001 0000)
    #[inline(always)]
    pub fn sram1(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD0::Sram1)
    }
    ///Boot from SRAM2 (0x2003 C000)
    #[inline(always)]
    pub fn sram2(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD0::Sram2)
    }
    ///Boot from specified address (granularity of 16KB)
    #[inline(always)]
    pub fn boot_address(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD0::BootAddress)
    }
}
/**Boot base address when Boot pin =1

Value on reset: 4095*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BOOT_ADD1 {
    ///0: Boot from ITCM RAM (0x0000 0000)
    ItcmRam = 0,
    ///64: Boot from System memory bootloader (0x0010 0000)
    SystemMemoryBootloader = 64,
    ///128: Boot from Flash on ITCM interface (0x0020 0000)
    FlashItcmInterface = 128,
    ///8192: Boot from Flash on AXIM interface (0x0800 0000)
    FlashAximInterface = 8192,
    ///32768: Boot from DTCM RAM (0x2000 0000)
    DtcmRam = 32768,
    ///32772: Boot from SRAM1 (0x2001 0000)
    Sram1 = 32772,
    ///32787: Boot from SRAM2 (0x2003 C000)
    Sram2 = 32787,
    ///1: Boot from specified address (granularity of 16KB)
    BootAddress = 1,
}
impl From<BOOT_ADD1> for u16 {
    #[inline(always)]
    fn from(variant: BOOT_ADD1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOOT_ADD1 {
    type Ux = u16;
}
impl crate::IsEnum for BOOT_ADD1 {}
///Field `BOOT_ADD1` reader - Boot base address when Boot pin =1
pub type BOOT_ADD1_R = crate::FieldReader<BOOT_ADD1>;
impl BOOT_ADD1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOOT_ADD1 {
        match self.bits {
            0 => BOOT_ADD1::ItcmRam,
            64 => BOOT_ADD1::SystemMemoryBootloader,
            128 => BOOT_ADD1::FlashItcmInterface,
            8192 => BOOT_ADD1::FlashAximInterface,
            32768 => BOOT_ADD1::DtcmRam,
            32772 => BOOT_ADD1::Sram1,
            32787 => BOOT_ADD1::Sram2,
            _ => BOOT_ADD1::BootAddress,
        }
    }
    ///Boot from ITCM RAM (0x0000 0000)
    #[inline(always)]
    pub fn is_itcm_ram(&self) -> bool {
        *self == BOOT_ADD1::ItcmRam
    }
    ///Boot from System memory bootloader (0x0010 0000)
    #[inline(always)]
    pub fn is_system_memory_bootloader(&self) -> bool {
        *self == BOOT_ADD1::SystemMemoryBootloader
    }
    ///Boot from Flash on ITCM interface (0x0020 0000)
    #[inline(always)]
    pub fn is_flash_itcm_interface(&self) -> bool {
        *self == BOOT_ADD1::FlashItcmInterface
    }
    ///Boot from Flash on AXIM interface (0x0800 0000)
    #[inline(always)]
    pub fn is_flash_axim_interface(&self) -> bool {
        *self == BOOT_ADD1::FlashAximInterface
    }
    ///Boot from DTCM RAM (0x2000 0000)
    #[inline(always)]
    pub fn is_dtcm_ram(&self) -> bool {
        *self == BOOT_ADD1::DtcmRam
    }
    ///Boot from SRAM1 (0x2001 0000)
    #[inline(always)]
    pub fn is_sram1(&self) -> bool {
        *self == BOOT_ADD1::Sram1
    }
    ///Boot from SRAM2 (0x2003 C000)
    #[inline(always)]
    pub fn is_sram2(&self) -> bool {
        *self == BOOT_ADD1::Sram2
    }
    ///Boot from specified address (granularity of 16KB)
    #[inline(always)]
    pub fn is_boot_address(&self) -> bool {
        matches!(self.variant(), BOOT_ADD1::BootAddress)
    }
}
///Field `BOOT_ADD1` writer - Boot base address when Boot pin =1
pub type BOOT_ADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, BOOT_ADD1, crate::Safe>;
impl<'a, REG> BOOT_ADD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Boot from ITCM RAM (0x0000 0000)
    #[inline(always)]
    pub fn itcm_ram(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD1::ItcmRam)
    }
    ///Boot from System memory bootloader (0x0010 0000)
    #[inline(always)]
    pub fn system_memory_bootloader(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD1::SystemMemoryBootloader)
    }
    ///Boot from Flash on ITCM interface (0x0020 0000)
    #[inline(always)]
    pub fn flash_itcm_interface(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD1::FlashItcmInterface)
    }
    ///Boot from Flash on AXIM interface (0x0800 0000)
    #[inline(always)]
    pub fn flash_axim_interface(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD1::FlashAximInterface)
    }
    ///Boot from DTCM RAM (0x2000 0000)
    #[inline(always)]
    pub fn dtcm_ram(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD1::DtcmRam)
    }
    ///Boot from SRAM1 (0x2001 0000)
    #[inline(always)]
    pub fn sram1(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD1::Sram1)
    }
    ///Boot from SRAM2 (0x2003 C000)
    #[inline(always)]
    pub fn sram2(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD1::Sram2)
    }
    ///Boot from specified address (granularity of 16KB)
    #[inline(always)]
    pub fn boot_address(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_ADD1::BootAddress)
    }
}
impl R {
    ///Bits 0:15 - Boot base address when Boot pin =0
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Boot base address when Boot pin =1
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR1")
            .field("boot_add0", &self.boot_add0())
            .field("boot_add1", &self.boot_add1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Boot base address when Boot pin =0
    #[inline(always)]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W<'_, OPTCR1rs> {
        BOOT_ADD0_W::new(self, 0)
    }
    ///Bits 16:31 - Boot base address when Boot pin =1
    #[inline(always)]
    pub fn boot_add1(&mut self) -> BOOT_ADD1_W<'_, OPTCR1rs> {
        BOOT_ADD1_W::new(self, 16)
    }
}
/**Flash option control register 1

You can [`read`](crate::Reg::read) this register and get [`optcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FLASH:OPTCR1)*/
pub struct OPTCR1rs;
impl crate::RegisterSpec for OPTCR1rs {
    type Ux = u32;
}
///`read()` method returns [`optcr1::R`](R) reader structure
impl crate::Readable for OPTCR1rs {}
///`write(|w| ..)` method takes [`optcr1::W`](W) writer structure
impl crate::Writable for OPTCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTCR1 to value 0x0fff_0000
impl crate::Resettable for OPTCR1rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
