///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
/**Memory mapping selection bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE {
    ///0: Main Flash memory mapped at 0x0000_0000
    MainFlash = 0,
    ///1: System Flash memory mapped at 0x0000_0000
    SystemFlash = 1,
    ///3: Embedded SRAM mapped at 0x0000_0000
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
impl crate::IsEnum for MEM_MODE {}
///Field `MEM_MODE` reader - Memory mapping selection bits
pub type MEM_MODE_R = crate::FieldReader<MEM_MODE>;
impl MEM_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MEM_MODE> {
        match self.bits {
            0 => Some(MEM_MODE::MainFlash),
            1 => Some(MEM_MODE::SystemFlash),
            3 => Some(MEM_MODE::Sram),
            _ => None,
        }
    }
    ///Main Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE::MainFlash
    }
    ///System Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE::SystemFlash
    }
    ///Embedded SRAM mapped at 0x0000_0000
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE::Sram
    }
}
///Field `MEM_MODE` writer - Memory mapping selection bits
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MEM_MODE>;
impl<'a, REG> MEM_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Main Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::MainFlash)
    }
    ///System Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::SystemFlash)
    }
    ///Embedded SRAM mapped at 0x0000_0000
    #[inline(always)]
    pub fn sram(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::Sram)
    }
}
/**User bank swapping

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UFB {
    ///0: Flash Program memory Bank 1 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 1 at 0x0808 0000 (aliased at 0x0008 0000 if MEM_MODE=00)
    Bank1 = 0,
    ///1: Flash Program memory Bank 2 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 2 at 0x0808 0000 (and aliased at 0x0008 0000 if MEM_MODE=00)
    Bank2 = 1,
}
impl From<UFB> for bool {
    #[inline(always)]
    fn from(variant: UFB) -> Self {
        variant as u8 != 0
    }
}
///Field `UFB` reader - User bank swapping
pub type UFB_R = crate::BitReader<UFB>;
impl UFB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UFB {
        match self.bits {
            false => UFB::Bank1,
            true => UFB::Bank2,
        }
    }
    ///Flash Program memory Bank 1 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 1 at 0x0808 0000 (aliased at 0x0008 0000 if MEM_MODE=00)
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == UFB::Bank1
    }
    ///Flash Program memory Bank 2 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 2 at 0x0808 0000 (and aliased at 0x0008 0000 if MEM_MODE=00)
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == UFB::Bank2
    }
}
///Field `UFB` writer - User bank swapping
pub type UFB_W<'a, REG> = crate::BitWriter<'a, REG, UFB>;
impl<'a, REG> UFB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash Program memory Bank 1 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 1 at 0x0808 0000 (aliased at 0x0008 0000 if MEM_MODE=00)
    #[inline(always)]
    pub fn bank1(self) -> &'a mut crate::W<REG> {
        self.variant(UFB::Bank1)
    }
    ///Flash Program memory Bank 2 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 2 at 0x0808 0000 (and aliased at 0x0008 0000 if MEM_MODE=00)
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(UFB::Bank2)
    }
}
/**Boot mode selected by the boot pins status bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOT_MODE {
    ///0: Main Flash memory boot mode
    MainFlash = 0,
    ///1: System Flash memory boot mode
    SystemFlash = 1,
    ///3: Embedded SRAM boot mode
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
impl crate::IsEnum for BOOT_MODE {}
///Field `BOOT_MODE` reader - Boot mode selected by the boot pins status bits
pub type BOOT_MODE_R = crate::FieldReader<BOOT_MODE>;
impl BOOT_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BOOT_MODE> {
        match self.bits {
            0 => Some(BOOT_MODE::MainFlash),
            1 => Some(BOOT_MODE::SystemFlash),
            3 => Some(BOOT_MODE::Sram),
            _ => None,
        }
    }
    ///Main Flash memory boot mode
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == BOOT_MODE::MainFlash
    }
    ///System Flash memory boot mode
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == BOOT_MODE::SystemFlash
    }
    ///Embedded SRAM boot mode
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == BOOT_MODE::Sram
    }
}
impl R {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - User bank swapping
    #[inline(always)]
    pub fn ufb(&self) -> UFB_R {
        UFB_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:9 - Boot mode selected by the boot pins status bits
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("boot_mode", &self.boot_mode())
            .field("mem_mode", &self.mem_mode())
            .field("ufb", &self.ufb())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    ///Bit 3 - User bank swapping
    #[inline(always)]
    pub fn ufb(&mut self) -> UFB_W<CFGR1rs> {
        UFB_W::new(self, 3)
    }
}
/**SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#SYSCFG:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
