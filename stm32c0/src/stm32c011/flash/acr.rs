///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
///Field `LATENCY` reader - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read.
pub type LATENCY_R = crate::FieldReader;
///Field `LATENCY` writer - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read.
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PRFTEN` reader - CPU Prefetch enable
pub type PRFTEN_R = crate::BitReader;
///Field `PRFTEN` writer - CPU Prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICEN` reader - CPU Instruction cache enable
pub type ICEN_R = crate::BitReader;
///Field `ICEN` writer - CPU Instruction cache enable
pub type ICEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICRST` reader - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled.
pub type ICRST_R = crate::BitReader;
///Field `ICRST` writer - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled.
pub type ICRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMPTY` reader - Main flash memory area empty This bit indicates whether the first location of the Main flash memory area was read as erased or as programmed during OBL. It is not affected by the system reset. Software may need to change this bit value after a flash memory program or erase operation. The bit can be set and reset by software.
pub type EMPTY_R = crate::BitReader;
///Field `EMPTY` writer - Main flash memory area empty This bit indicates whether the first location of the Main flash memory area was read as erased or as programmed during OBL. It is not affected by the system reset. Software may need to change this bit value after a flash memory program or erase operation. The bit can be set and reset by software.
pub type EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_SWEN` reader - Debug access software enable Software may use this bit to enable/disable the debugger read access.
pub type DBG_SWEN_R = crate::BitReader;
///Field `DBG_SWEN` writer - Debug access software enable Software may use this bit to enable/disable the debugger read access.
pub type DBG_SWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read.
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - CPU Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU Instruction cache enable
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled.
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - Main flash memory area empty This bit indicates whether the first location of the Main flash memory area was read as erased or as programmed during OBL. It is not affected by the system reset. Software may need to change this bit value after a flash memory program or erase operation. The bit can be set and reset by software.
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Debug access software enable Software may use this bit to enable/disable the debugger read access.
    #[inline(always)]
    pub fn dbg_swen(&self) -> DBG_SWEN_R {
        DBG_SWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("latency", &self.latency())
            .field("prften", &self.prften())
            .field("icen", &self.icen())
            .field("icrst", &self.icrst())
            .field("empty", &self.empty())
            .field("dbg_swen", &self.dbg_swen())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Flash memory access latency The value in this bitfield represents the number of CPU wait states when accessing the flash memory. Other: Reserved A new write into the bitfield becomes effective when it returns the same value upon read.
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 8 - CPU Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    ///Bit 9 - CPU Instruction cache enable
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W<ACRrs> {
        ICEN_W::new(self, 9)
    }
    ///Bit 11 - CPU Instruction cache reset This bit can be written only when the instruction cache is disabled.
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W<ACRrs> {
        ICRST_W::new(self, 11)
    }
    ///Bit 16 - Main flash memory area empty This bit indicates whether the first location of the Main flash memory area was read as erased or as programmed during OBL. It is not affected by the system reset. Software may need to change this bit value after a flash memory program or erase operation. The bit can be set and reset by software.
    #[inline(always)]
    pub fn empty(&mut self) -> EMPTY_W<ACRrs> {
        EMPTY_W::new(self, 16)
    }
    ///Bit 18 - Debug access software enable Software may use this bit to enable/disable the debugger read access.
    #[inline(always)]
    pub fn dbg_swen(&mut self) -> DBG_SWEN_W<ACRrs> {
        DBG_SWEN_W::new(self, 18)
    }
}
/**FLASH access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#FLASH:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ACR to value 0x0004_0600
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0x0004_0600;
}
