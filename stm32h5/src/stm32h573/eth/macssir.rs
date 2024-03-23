#[doc = "Register `MACSSIR` reader"]
pub type R = crate::R<MACSSIRrs>;
#[doc = "Register `MACSSIR` writer"]
pub type W = crate::W<MACSSIRrs>;
#[doc = "Field `SSINC` reader - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the subsecond register. For example, when the PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time Nanoseconds register has an accuracy of 1 ns \\[TSCTRLSSR bit is set in Timestamp control Register (ETH_MACTSCR)\\]. When TSCTRLSSR is cleared, the Nanoseconds register has a resolution of ~0.465 ns. In this case, you should program a value of 43 (0x2B) which is derived by 20 ns/0.465."]
pub type SSINC_R = crate::FieldReader;
#[doc = "Field `SSINC` writer - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the subsecond register. For example, when the PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time Nanoseconds register has an accuracy of 1 ns \\[TSCTRLSSR bit is set in Timestamp control Register (ETH_MACTSCR)\\]. When TSCTRLSSR is cleared, the Nanoseconds register has a resolution of ~0.465 ns. In this case, you should program a value of 43 (0x2B) which is derived by 20 ns/0.465."]
pub type SSINC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the subsecond register. For example, when the PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time Nanoseconds register has an accuracy of 1 ns \\[TSCTRLSSR bit is set in Timestamp control Register (ETH_MACTSCR)\\]. When TSCTRLSSR is cleared, the Nanoseconds register has a resolution of ~0.465 ns. In this case, you should program a value of 43 (0x2B) which is derived by 20 ns/0.465."]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the subsecond register. For example, when the PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time Nanoseconds register has an accuracy of 1 ns \\[TSCTRLSSR bit is set in Timestamp control Register (ETH_MACTSCR)\\]. When TSCTRLSSR is cleared, the Nanoseconds register has a resolution of ~0.465 ns. In this case, you should program a value of 43 (0x2B) which is derived by 20 ns/0.465."]
    #[inline(always)]
    #[must_use]
    pub fn ssinc(&mut self) -> SSINC_W<MACSSIRrs> {
        SSINC_W::new(self, 16)
    }
}
#[doc = "Subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macssir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macssir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSSIRrs;
impl crate::RegisterSpec for MACSSIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macssir::R`](R) reader structure"]
impl crate::Readable for MACSSIRrs {}
#[doc = "`write(|w| ..)` method takes [`macssir::W`](W) writer structure"]
impl crate::Writable for MACSSIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACSSIR to value 0"]
impl crate::Resettable for MACSSIRrs {
    const RESET_VALUE: u32 = 0;
}
