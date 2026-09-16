///Register `DCR` reader
pub type R = crate::R<DCRrs>;
///Register `DCR` writer
pub type W = crate::W<DCRrs>;
/**indicates the level that clk takes between command

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKMODE {
    ///0: CLK must stay low while nCS is high (chip select released). This is referred to as mode 0.
    Mode0 = 0,
    ///1: CLK must stay high while nCS is high (chip select released). This is referred to as mode 3.
    Mode3 = 1,
}
impl From<CKMODE> for bool {
    #[inline(always)]
    fn from(variant: CKMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `CKMODE` reader - indicates the level that clk takes between command
pub type CKMODE_R = crate::BitReader<CKMODE>;
impl CKMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKMODE {
        match self.bits {
            false => CKMODE::Mode0,
            true => CKMODE::Mode3,
        }
    }
    ///CLK must stay low while nCS is high (chip select released). This is referred to as mode 0.
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == CKMODE::Mode0
    }
    ///CLK must stay high while nCS is high (chip select released). This is referred to as mode 3.
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == CKMODE::Mode3
    }
}
///Field `CKMODE` writer - indicates the level that clk takes between command
pub type CKMODE_W<'a, REG> = crate::BitWriter<'a, REG, CKMODE>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CLK must stay low while nCS is high (chip select released). This is referred to as mode 0.
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Mode0)
    }
    ///CLK must stay high while nCS is high (chip select released). This is referred to as mode 3.
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE::Mode3)
    }
}
///Field `CSHT` reader - Chip select high time CSHT+1 defines the minimum number of CLK cycles which the chip select (nCS) must remain high between commands issued to the Flash memory. ... This field can be modified only when BUSY = 0.
pub type CSHT_R = crate::FieldReader;
///Field `CSHT` writer - Chip select high time CSHT+1 defines the minimum number of CLK cycles which the chip select (nCS) must remain high between commands issued to the Flash memory. ... This field can be modified only when BUSY = 0.
pub type CSHT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
///Field `FSIZE` reader - Flash memory size This field defines the size of external memory using the following formula: Number of bytes in Flash memory = 2\[FSIZE+1\] FSIZE+1 is effectively the number of address bits required to address the Flash memory. The Flash memory capacity can be up to 4GB (addressed using 32 bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256MB. If DFM = 1, FSIZE indicates the total capacity of the two Flash memories together. This field can be modified only when BUSY = 0.
pub type FSIZE_R = crate::FieldReader;
///Field `FSIZE` writer - Flash memory size This field defines the size of external memory using the following formula: Number of bytes in Flash memory = 2\[FSIZE+1\] FSIZE+1 is effectively the number of address bits required to address the Flash memory. The Flash memory capacity can be up to 4GB (addressed using 32 bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256MB. If DFM = 1, FSIZE indicates the total capacity of the two Flash memories together. This field can be modified only when BUSY = 0.
pub type FSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    ///Bit 0 - indicates the level that clk takes between command
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:10 - Chip select high time CSHT+1 defines the minimum number of CLK cycles which the chip select (nCS) must remain high between commands issued to the Flash memory. ... This field can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 16:20 - Flash memory size This field defines the size of external memory using the following formula: Number of bytes in Flash memory = 2\[FSIZE+1\] FSIZE+1 is effectively the number of address bits required to address the Flash memory. The Flash memory capacity can be up to 4GB (addressed using 32 bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256MB. If DFM = 1, FSIZE indicates the total capacity of the two Flash memories together. This field can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn fsize(&self) -> FSIZE_R {
        FSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCR")
            .field("ckmode", &self.ckmode())
            .field("csht", &self.csht())
            .field("fsize", &self.fsize())
            .finish()
    }
}
impl W {
    ///Bit 0 - indicates the level that clk takes between command
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<'_, DCRrs> {
        CKMODE_W::new(self, 0)
    }
    ///Bits 8:10 - Chip select high time CSHT+1 defines the minimum number of CLK cycles which the chip select (nCS) must remain high between commands issued to the Flash memory. ... This field can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W<'_, DCRrs> {
        CSHT_W::new(self, 8)
    }
    ///Bits 16:20 - Flash memory size This field defines the size of external memory using the following formula: Number of bytes in Flash memory = 2\[FSIZE+1\] FSIZE+1 is effectively the number of address bits required to address the Flash memory. The Flash memory capacity can be up to 4GB (addressed using 32 bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256MB. If DFM = 1, FSIZE indicates the total capacity of the two Flash memories together. This field can be modified only when BUSY = 0.
    #[inline(always)]
    pub fn fsize(&mut self) -> FSIZE_W<'_, DCRrs> {
        FSIZE_W::new(self, 16)
    }
}
/**QUADSPI device configuration register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#QUADSPI:DCR)*/
pub struct DCRrs;
impl crate::RegisterSpec for DCRrs {
    type Ux = u32;
}
///`read()` method returns [`dcr::R`](R) reader structure
impl crate::Readable for DCRrs {}
///`write(|w| ..)` method takes [`dcr::W`](W) writer structure
impl crate::Writable for DCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCR to value 0
impl crate::Resettable for DCRrs {}
