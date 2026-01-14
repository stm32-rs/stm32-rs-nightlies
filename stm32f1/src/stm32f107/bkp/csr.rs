///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**Clear Tamper event

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEW {
    ///1: Reset the TEF Tamper event flag (and the Tamper detector)
    Reset = 1,
}
impl From<CTEW> for bool {
    #[inline(always)]
    fn from(variant: CTEW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTE` writer - Clear Tamper event
pub type CTE_W<'a, REG> = crate::BitWriter<'a, REG, CTEW>;
impl<'a, REG> CTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the TEF Tamper event flag (and the Tamper detector)
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CTEW::Reset)
    }
}
/**Clear Tamper Interrupt

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIW {
    ///1: Clear the Tamper interrupt and the TIF Tamper interrupt flag
    Clear = 1,
}
impl From<CTIW> for bool {
    #[inline(always)]
    fn from(variant: CTIW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTI` writer - Clear Tamper Interrupt
pub type CTI_W<'a, REG> = crate::BitWriter<'a, REG, CTIW>;
impl<'a, REG> CTI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the Tamper interrupt and the TIF Tamper interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTIW::Clear)
    }
}
/**Tamper Pin interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIE {
    ///0: Tamper interrupt disabled
    Disabled = 0,
    ///1: Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register
    Enabled = 1,
}
impl From<TPIE> for bool {
    #[inline(always)]
    fn from(variant: TPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TPIE` reader - Tamper Pin interrupt enable
pub type TPIE_R = crate::BitReader<TPIE>;
impl TPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TPIE {
        match self.bits {
            false => TPIE::Disabled,
            true => TPIE::Enabled,
        }
    }
    ///Tamper interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPIE::Disabled
    }
    ///Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPIE::Enabled
    }
}
///Field `TPIE` writer - Tamper Pin interrupt enable
pub type TPIE_W<'a, REG> = crate::BitWriter<'a, REG, TPIE>;
impl<'a, REG> TPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tamper interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPIE::Disabled)
    }
    ///Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPIE::Enabled)
    }
}
///Field `TEF` reader - Tamper Event Flag
pub type TEF_R = crate::BitReader;
///Field `TIF` reader - Tamper Interrupt Flag
pub type TIF_R = crate::BitReader;
impl R {
    ///Bit 2 - Tamper Pin interrupt enable
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Tamper Event Flag
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Tamper Interrupt Flag
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("tpie", &self.tpie())
            .field("tef", &self.tef())
            .field("tif", &self.tif())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear Tamper event
    #[inline(always)]
    pub fn cte(&mut self) -> CTE_W<'_, CSRrs> {
        CTE_W::new(self, 0)
    }
    ///Bit 1 - Clear Tamper Interrupt
    #[inline(always)]
    pub fn cti(&mut self) -> CTI_W<'_, CSRrs> {
        CTI_W::new(self, 1)
    }
    ///Bit 2 - Tamper Pin interrupt enable
    #[inline(always)]
    pub fn tpie(&mut self) -> TPIE_W<'_, CSRrs> {
        TPIE_W::new(self, 2)
    }
}
/**BKP_CSR control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#BKP:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
