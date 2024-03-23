#[doc = "Register `ICSR` reader"]
pub type R = crate::R<ICSRrs>;
#[doc = "Register `ICSR` writer"]
pub type W = crate::W<ICSRrs>;
#[doc = "Field `WUTWF` reader - Wakeup timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
pub type WUTWF_R = crate::BitReader;
#[doc = "Field `SHPF` reader - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
pub type SHPF_R = crate::BitReader;
#[doc = "Field `INITS` reader - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state)."]
pub type INITS_R = crate::BitReader;
#[doc = "Field `RSF` reader - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITF` reader - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated."]
pub type INITF_R = crate::BitReader;
#[doc = "Field `INIT` reader - Initialization mode"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization mode"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIN` reader - Binary mode"]
pub type BIN_R = crate::FieldReader;
#[doc = "Field `BIN` writer - Binary mode"]
pub type BIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BCDU` reader - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
pub type BCDU_R = crate::FieldReader;
#[doc = "Field `BCDU` writer - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
pub type BCDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RECALPF` reader - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to ."]
pub type RECALPF_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Wakeup timer write flag This bit is set by hardware when WUT value can be changed, after the WUTE bit has been set to 0 in RTC_CR. It is cleared by hardware in initialization mode."]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending This flag is set by hardware as soon as a shift operation is initiated by a write to the RTC_SHIFTR register. It is cleared by hardware when the corresponding shift operation has been executed. Writing to the SHPF bit has no effect."]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag This bit is set by hardware when the calendar year field is different from 0 (Backup domain reset state)."]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag When this bit is set to 1, the RTC is in initialization state, and the time, date and prescaler registers can be updated."]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Binary mode"]
    #[inline(always)]
    pub fn bin(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
    #[inline(always)]
    pub fn bcdu(&self) -> BCDU_R {
        BCDU_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 16 - Recalibration pending Flag The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0. Refer to ."]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Registers synchronization flag This bit is set by hardware each time the calendar registers are copied into the shadow registers (RTC_SSR, RTC_TR and RTC_DR). This bit is cleared by hardware in initialization mode, while a shift operation is pending (SHPF = 1), or when in bypass shadow register mode (BYPSHAD = 1). This bit can also be cleared by software. It is cleared either by software or by hardware in initialization mode."]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<ICSRrs> {
        RSF_W::new(self, 5)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<ICSRrs> {
        INIT_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Binary mode"]
    #[inline(always)]
    #[must_use]
    pub fn bin(&mut self) -> BIN_W<ICSRrs> {
        BIN_W::new(self, 8)
    }
    #[doc = "Bits 10:12 - BCD update (BIN = 10 or 11) In mixed mode when both BCD calendar and binary extended counter are used (BIN = 10 or 11), the calendar second is incremented using the SSR Least Significant Bits."]
    #[inline(always)]
    #[must_use]
    pub fn bcdu(&mut self) -> BCDU_W<ICSRrs> {
        BCDU_W::new(self, 10)
    }
}
#[doc = "RTC initialization control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICSRrs;
impl crate::RegisterSpec for ICSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsr::R`](R) reader structure"]
impl crate::Readable for ICSRrs {}
#[doc = "`write(|w| ..)` method takes [`icsr::W`](W) writer structure"]
impl crate::Writable for ICSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSR to value 0x07"]
impl crate::Resettable for ICSRrs {
    const RESET_VALUE: u32 = 0x07;
}
