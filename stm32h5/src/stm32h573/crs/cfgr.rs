#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `RELOAD` reader - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to for more details about counter behavior."]
pub type RELOAD_R = crate::FieldReader<u16>;
#[doc = "Field `RELOAD` writer - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to for more details about counter behavior."]
pub type RELOAD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
#[doc = "Field `FELIM` reader - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to for more details about FECAP evaluation."]
pub type FELIM_R = crate::FieldReader;
#[doc = "Field `FELIM` writer - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to for more details about FECAP evaluation."]
pub type FELIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCDIV {
    #[doc = "0: SYNC not divided"]
    NotDivided = 0,
    #[doc = "1: SYNC divided by 2"]
    DivideBy2 = 1,
    #[doc = "2: SYNC divided by 4"]
    DivideBy4 = 2,
    #[doc = "3: SYNC divided by 8"]
    DivideBy8 = 3,
    #[doc = "4: SYNC divided by 16"]
    DivideBy16 = 4,
    #[doc = "5: SYNC divided by 32"]
    DivideBy32 = 5,
    #[doc = "6: SYNC divided by 64"]
    DivideBy64 = 6,
    #[doc = "7: SYNC divided by 128"]
    DivideBy128 = 7,
}
impl From<SYNCDIV> for u8 {
    #[inline(always)]
    fn from(variant: SYNCDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCDIV {
    type Ux = u8;
}
#[doc = "Field `SYNCDIV` reader - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
pub type SYNCDIV_R = crate::FieldReader<SYNCDIV>;
impl SYNCDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCDIV {
        match self.bits {
            0 => SYNCDIV::NotDivided,
            1 => SYNCDIV::DivideBy2,
            2 => SYNCDIV::DivideBy4,
            3 => SYNCDIV::DivideBy8,
            4 => SYNCDIV::DivideBy16,
            5 => SYNCDIV::DivideBy32,
            6 => SYNCDIV::DivideBy64,
            7 => SYNCDIV::DivideBy128,
            _ => unreachable!(),
        }
    }
    #[doc = "SYNC not divided"]
    #[inline(always)]
    pub fn is_not_divided(&self) -> bool {
        *self == SYNCDIV::NotDivided
    }
    #[doc = "SYNC divided by 2"]
    #[inline(always)]
    pub fn is_divide_by2(&self) -> bool {
        *self == SYNCDIV::DivideBy2
    }
    #[doc = "SYNC divided by 4"]
    #[inline(always)]
    pub fn is_divide_by4(&self) -> bool {
        *self == SYNCDIV::DivideBy4
    }
    #[doc = "SYNC divided by 8"]
    #[inline(always)]
    pub fn is_divide_by8(&self) -> bool {
        *self == SYNCDIV::DivideBy8
    }
    #[doc = "SYNC divided by 16"]
    #[inline(always)]
    pub fn is_divide_by16(&self) -> bool {
        *self == SYNCDIV::DivideBy16
    }
    #[doc = "SYNC divided by 32"]
    #[inline(always)]
    pub fn is_divide_by32(&self) -> bool {
        *self == SYNCDIV::DivideBy32
    }
    #[doc = "SYNC divided by 64"]
    #[inline(always)]
    pub fn is_divide_by64(&self) -> bool {
        *self == SYNCDIV::DivideBy64
    }
    #[doc = "SYNC divided by 128"]
    #[inline(always)]
    pub fn is_divide_by128(&self) -> bool {
        *self == SYNCDIV::DivideBy128
    }
}
#[doc = "Field `SYNCDIV` writer - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
pub type SYNCDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SYNCDIV>;
impl<'a, REG> SYNCDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYNC not divided"]
    #[inline(always)]
    pub fn not_divided(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::NotDivided)
    }
    #[doc = "SYNC divided by 2"]
    #[inline(always)]
    pub fn divide_by2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::DivideBy2)
    }
    #[doc = "SYNC divided by 4"]
    #[inline(always)]
    pub fn divide_by4(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::DivideBy4)
    }
    #[doc = "SYNC divided by 8"]
    #[inline(always)]
    pub fn divide_by8(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::DivideBy8)
    }
    #[doc = "SYNC divided by 16"]
    #[inline(always)]
    pub fn divide_by16(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::DivideBy16)
    }
    #[doc = "SYNC divided by 32"]
    #[inline(always)]
    pub fn divide_by32(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::DivideBy32)
    }
    #[doc = "SYNC divided by 64"]
    #[inline(always)]
    pub fn divide_by64(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::DivideBy64)
    }
    #[doc = "SYNC divided by 128"]
    #[inline(always)]
    pub fn divide_by128(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCDIV::DivideBy128)
    }
}
#[doc = "SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see CRS internal input/output signals for STM32U575/585): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCSRC {
    #[doc = "0: GPIO AF (crs_sync_in_1) selected as SYNC signal source"]
    GpioAf = 0,
    #[doc = "1: LSE (crs_sync_in_2) selected as SYNC signal source"]
    Lse = 1,
    #[doc = "2: USB SOF (crs_sync_in_3) selected as SYNC signal source"]
    UsbSof = 2,
}
impl From<SYNCSRC> for u8 {
    #[inline(always)]
    fn from(variant: SYNCSRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCSRC {
    type Ux = u8;
}
#[doc = "Field `SYNCSRC` reader - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see CRS internal input/output signals for STM32U575/585): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
pub type SYNCSRC_R = crate::FieldReader<SYNCSRC>;
impl SYNCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCSRC> {
        match self.bits {
            0 => Some(SYNCSRC::GpioAf),
            1 => Some(SYNCSRC::Lse),
            2 => Some(SYNCSRC::UsbSof),
            _ => None,
        }
    }
    #[doc = "GPIO AF (crs_sync_in_1) selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_gpio_af(&self) -> bool {
        *self == SYNCSRC::GpioAf
    }
    #[doc = "LSE (crs_sync_in_2) selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == SYNCSRC::Lse
    }
    #[doc = "USB SOF (crs_sync_in_3) selected as SYNC signal source"]
    #[inline(always)]
    pub fn is_usb_sof(&self) -> bool {
        *self == SYNCSRC::UsbSof
    }
}
#[doc = "Field `SYNCSRC` writer - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see CRS internal input/output signals for STM32U575/585): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
pub type SYNCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCSRC>;
impl<'a, REG> SYNCSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO AF (crs_sync_in_1) selected as SYNC signal source"]
    #[inline(always)]
    pub fn gpio_af(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::GpioAf)
    }
    #[doc = "LSE (crs_sync_in_2) selected as SYNC signal source"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::Lse)
    }
    #[doc = "USB SOF (crs_sync_in_3) selected as SYNC signal source"]
    #[inline(always)]
    pub fn usb_sof(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCSRC::UsbSof)
    }
}
#[doc = "SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCPOL {
    #[doc = "0: SYNC active on rising edge"]
    RisingEdge = 0,
    #[doc = "1: SYNC active on falling edge"]
    FallingEdge = 1,
}
impl From<SYNCPOL> for bool {
    #[inline(always)]
    fn from(variant: SYNCPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCPOL` reader - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
pub type SYNCPOL_R = crate::BitReader<SYNCPOL>;
impl SYNCPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCPOL {
        match self.bits {
            false => SYNCPOL::RisingEdge,
            true => SYNCPOL::FallingEdge,
        }
    }
    #[doc = "SYNC active on rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SYNCPOL::RisingEdge
    }
    #[doc = "SYNC active on falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SYNCPOL::FallingEdge
    }
}
#[doc = "Field `SYNCPOL` writer - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
pub type SYNCPOL_W<'a, REG> = crate::BitWriter<'a, REG, SYNCPOL>;
impl<'a, REG> SYNCPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYNC active on rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPOL::RisingEdge)
    }
    #[doc = "SYNC active on falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPOL::FallingEdge)
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to for more details about counter behavior."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to for more details about FECAP evaluation."]
    #[inline(always)]
    pub fn felim(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
    #[inline(always)]
    pub fn syncdiv(&self) -> SYNCDIV_R {
        SYNCDIV_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see CRS internal input/output signals for STM32U575/585): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to for more details about counter behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<CFGRrs> {
        RELOAD_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to for more details about FECAP evaluation."]
    #[inline(always)]
    #[must_use]
    pub fn felim(&mut self) -> FELIM_W<CFGRrs> {
        FELIM_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
    #[inline(always)]
    #[must_use]
    pub fn syncdiv(&mut self) -> SYNCDIV_W<CFGRrs> {
        SYNCDIV_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source (see CRS internal input/output signals for STM32U575/585): Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF is not generated by the host. No SYNC signal is therefore provided to the CRS to calibrate the HSI48 oscillator on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs must be used as SYNC signal."]
    #[inline(always)]
    #[must_use]
    pub fn syncsrc(&mut self) -> SYNCSRC_W<CFGRrs> {
        SYNCSRC_W::new(self, 28)
    }
    #[doc = "Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SYNCPOL_W<CFGRrs> {
        SYNCPOL_W::new(self, 31)
    }
}
#[doc = "CRS configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0x2022_bb7f"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0x2022_bb7f;
}
