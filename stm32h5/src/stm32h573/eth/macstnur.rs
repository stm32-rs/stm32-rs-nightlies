#[doc = "Register `MACSTNUR` reader"]
pub type R = crate::R<MACSTNURrs>;
#[doc = "Register `MACSTNUR` writer"]
pub type W = crate::W<MACSTNURrs>;
#[doc = "Field `TSSS` reader - Timestamp subseconds The value in this field is the subseconds part of the update. ADDSUB is 1: This field must be programmed with the complement of the subseconds part of the update value as described. ADDSUB is 0: This field must be programmed with the subseconds part of the update value, with an accuracy based on the TSCTRLSSR bit of the Timestamp control Register (ETH_MACTSCR). TSCTRLSSR field in the Timestamp control Register (ETH_MACTSCR)is 1: - The programmed value must be 10^9 – &lt;subsecond value>. - Each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF. TSCTRLSSR field in the Timestamp control Register (ETH_MACTSCR) is 0: - The programmed value must be 2^31�-�&lt;subsecond_value>. - Each bit represents an accuracy of 0.46�ns. For example, to subtract 2.000000001 seconds from the system time, then the TSSS field in the ETH_MACSTNUR register must be 0x7FFF_FFFF (that is, 2^31 – 1), when TSCTRLSSR bit in Timestamp control Register (ETH_MACTSCR) is reset and 0x3B9A_C9FF (that is, 10^9 –1), when TSCTRLSSR bit in Timestamp control Register (ETH_MACTSCR) is set. When the ADDSUB bit is set, TSSS\\[30:0\\]
field cannot be set to 0. The TSSS bitfield must be programmed to 0x7FFF FFFF (resulting in –0.46 ns) even if 0 ns must be subtracted. For example, to subtract 2.000000000 seconds from the system time, System time nanoseconds update register (ETH_MACSTNUR) must be 0xFFFF FFFF (ADDSUB = 1 and TSSS\\[30:0\\]
= 0) and the TSS field in the System time seconds update register (ETH_MACSTSUR) must be 0xFFFF FFFE (that is, 2^32 – 1)."]
pub type TSSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSSS` writer - Timestamp subseconds The value in this field is the subseconds part of the update. ADDSUB is 1: This field must be programmed with the complement of the subseconds part of the update value as described. ADDSUB is 0: This field must be programmed with the subseconds part of the update value, with an accuracy based on the TSCTRLSSR bit of the Timestamp control Register (ETH_MACTSCR). TSCTRLSSR field in the Timestamp control Register (ETH_MACTSCR)is 1: - The programmed value must be 10^9 – &lt;subsecond value>. - Each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF. TSCTRLSSR field in the Timestamp control Register (ETH_MACTSCR) is 0: - The programmed value must be 2^31�-�&lt;subsecond_value>. - Each bit represents an accuracy of 0.46�ns. For example, to subtract 2.000000001 seconds from the system time, then the TSSS field in the ETH_MACSTNUR register must be 0x7FFF_FFFF (that is, 2^31 – 1), when TSCTRLSSR bit in Timestamp control Register (ETH_MACTSCR) is reset and 0x3B9A_C9FF (that is, 10^9 –1), when TSCTRLSSR bit in Timestamp control Register (ETH_MACTSCR) is set. When the ADDSUB bit is set, TSSS\\[30:0\\]
field cannot be set to 0. The TSSS bitfield must be programmed to 0x7FFF FFFF (resulting in –0.46 ns) even if 0 ns must be subtracted. For example, to subtract 2.000000000 seconds from the system time, System time nanoseconds update register (ETH_MACSTNUR) must be 0xFFFF FFFF (ADDSUB = 1 and TSSS\\[30:0\\]
= 0) and the TSS field in the System time seconds update register (ETH_MACSTSUR) must be 0xFFFF FFFE (that is, 2^32 – 1)."]
pub type TSSS_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `ADDSUB` reader - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register. When this bit is reset, the time value is added with the contents of the update register."]
pub type ADDSUB_R = crate::BitReader;
#[doc = "Field `ADDSUB` writer - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register. When this bit is reset, the time value is added with the contents of the update register."]
pub type ADDSUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Timestamp subseconds The value in this field is the subseconds part of the update. ADDSUB is 1: This field must be programmed with the complement of the subseconds part of the update value as described. ADDSUB is 0: This field must be programmed with the subseconds part of the update value, with an accuracy based on the TSCTRLSSR bit of the Timestamp control Register (ETH_MACTSCR). TSCTRLSSR field in the Timestamp control Register (ETH_MACTSCR)is 1: - The programmed value must be 10^9 – &lt;subsecond value>. - Each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF. TSCTRLSSR field in the Timestamp control Register (ETH_MACTSCR) is 0: - The programmed value must be 2^31�-�&lt;subsecond_value>. - Each bit represents an accuracy of 0.46�ns. For example, to subtract 2.000000001 seconds from the system time, then the TSSS field in the ETH_MACSTNUR register must be 0x7FFF_FFFF (that is, 2^31 – 1), when TSCTRLSSR bit in Timestamp control Register (ETH_MACTSCR) is reset and 0x3B9A_C9FF (that is, 10^9 –1), when TSCTRLSSR bit in Timestamp control Register (ETH_MACTSCR) is set. When the ADDSUB bit is set, TSSS\\[30:0\\]
field cannot be set to 0. The TSSS bitfield must be programmed to 0x7FFF FFFF (resulting in –0.46 ns) even if 0 ns must be subtracted. For example, to subtract 2.000000000 seconds from the system time, System time nanoseconds update register (ETH_MACSTNUR) must be 0xFFFF FFFF (ADDSUB = 1 and TSSS\\[30:0\\]
= 0) and the TSS field in the System time seconds update register (ETH_MACSTSUR) must be 0xFFFF FFFE (that is, 2^32 – 1)."]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register. When this bit is reset, the time value is added with the contents of the update register."]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp subseconds The value in this field is the subseconds part of the update. ADDSUB is 1: This field must be programmed with the complement of the subseconds part of the update value as described. ADDSUB is 0: This field must be programmed with the subseconds part of the update value, with an accuracy based on the TSCTRLSSR bit of the Timestamp control Register (ETH_MACTSCR). TSCTRLSSR field in the Timestamp control Register (ETH_MACTSCR)is 1: - The programmed value must be 10^9 – &lt;subsecond value>. - Each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF. TSCTRLSSR field in the Timestamp control Register (ETH_MACTSCR) is 0: - The programmed value must be 2^31�-�&lt;subsecond_value>. - Each bit represents an accuracy of 0.46�ns. For example, to subtract 2.000000001 seconds from the system time, then the TSSS field in the ETH_MACSTNUR register must be 0x7FFF_FFFF (that is, 2^31 – 1), when TSCTRLSSR bit in Timestamp control Register (ETH_MACTSCR) is reset and 0x3B9A_C9FF (that is, 10^9 –1), when TSCTRLSSR bit in Timestamp control Register (ETH_MACTSCR) is set. When the ADDSUB bit is set, TSSS\\[30:0\\]
field cannot be set to 0. The TSSS bitfield must be programmed to 0x7FFF FFFF (resulting in –0.46 ns) even if 0 ns must be subtracted. For example, to subtract 2.000000000 seconds from the system time, System time nanoseconds update register (ETH_MACSTNUR) must be 0xFFFF FFFF (ADDSUB = 1 and TSSS\\[30:0\\]
= 0) and the TSS field in the System time seconds update register (ETH_MACSTSUR) must be 0xFFFF FFFE (that is, 2^32 – 1)."]
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TSSS_W<MACSTNURrs> {
        TSSS_W::new(self, 0)
    }
    #[doc = "Bit 31 - Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register. When this bit is reset, the time value is added with the contents of the update register."]
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> ADDSUB_W<MACSTNURrs> {
        ADDSUB_W::new(self, 31)
    }
}
#[doc = "System time nanoseconds update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstnur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macstnur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSTNURrs;
impl crate::RegisterSpec for MACSTNURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstnur::R`](R) reader structure"]
impl crate::Readable for MACSTNURrs {}
#[doc = "`write(|w| ..)` method takes [`macstnur::W`](W) writer structure"]
impl crate::Writable for MACSTNURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACSTNUR to value 0"]
impl crate::Resettable for MACSTNURrs {
    const RESET_VALUE: u32 = 0;
}
