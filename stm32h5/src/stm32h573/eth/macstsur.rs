#[doc = "Register `MACSTSUR` reader"]
pub type R = crate::R<MACSTSURrs>;
#[doc = "Register `MACSTSUR` writer"]
pub type W = crate::W<MACSTSURrs>;
#[doc = "Field `TSS` reader - Timestamp Seconds The value in this field is the seconds part of the update. When ADDSUB is reset, this field must be programmed with the seconds part of the update value. When ADDSUB is set, this field must be programmed with the complement of the seconds part of the update value. For example, to subtract 2.000000001 seconds from the system time, the TSS field in the ETH_MACSTSUR register must be 0xFFFF_FFFE (that is, 2^32 – 2). When the ADDSUB bit is set, TSSS\\[30:0\\]
field cannot be set to 0 in System time nanoseconds update register (ETH_MACSTNUR). The TSSS bitfield must be programmed to 0x7FFF FFFF (resulting in –0.46 ns) even if 0 ns must be subtracted. For example, to subtract 2.000000000 seconds from the system time, the TSS field in the System time seconds update register (ETH_MACSTSUR) must be 0xFFFF FFFE (that is, 2^32 – 1) and the System time nanoseconds update register (ETH_MACSTNUR) must be 0xFFFF FFFF (ADDSUB = 1 and TSSS\\[30:0\\]
field = 0x7FFF_FFFF)"]
pub type TSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSS` writer - Timestamp Seconds The value in this field is the seconds part of the update. When ADDSUB is reset, this field must be programmed with the seconds part of the update value. When ADDSUB is set, this field must be programmed with the complement of the seconds part of the update value. For example, to subtract 2.000000001 seconds from the system time, the TSS field in the ETH_MACSTSUR register must be 0xFFFF_FFFE (that is, 2^32 – 2). When the ADDSUB bit is set, TSSS\\[30:0\\]
field cannot be set to 0 in System time nanoseconds update register (ETH_MACSTNUR). The TSSS bitfield must be programmed to 0x7FFF FFFF (resulting in –0.46 ns) even if 0 ns must be subtracted. For example, to subtract 2.000000000 seconds from the system time, the TSS field in the System time seconds update register (ETH_MACSTSUR) must be 0xFFFF FFFE (that is, 2^32 – 1) and the System time nanoseconds update register (ETH_MACSTNUR) must be 0xFFFF FFFF (ADDSUB = 1 and TSSS\\[30:0\\]
field = 0x7FFF_FFFF)"]
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Seconds The value in this field is the seconds part of the update. When ADDSUB is reset, this field must be programmed with the seconds part of the update value. When ADDSUB is set, this field must be programmed with the complement of the seconds part of the update value. For example, to subtract 2.000000001 seconds from the system time, the TSS field in the ETH_MACSTSUR register must be 0xFFFF_FFFE (that is, 2^32 – 2). When the ADDSUB bit is set, TSSS\\[30:0\\]
field cannot be set to 0 in System time nanoseconds update register (ETH_MACSTNUR). The TSSS bitfield must be programmed to 0x7FFF FFFF (resulting in –0.46 ns) even if 0 ns must be subtracted. For example, to subtract 2.000000000 seconds from the system time, the TSS field in the System time seconds update register (ETH_MACSTSUR) must be 0xFFFF FFFE (that is, 2^32 – 1) and the System time nanoseconds update register (ETH_MACSTNUR) must be 0xFFFF FFFF (ADDSUB = 1 and TSSS\\[30:0\\]
field = 0x7FFF_FFFF)"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Seconds The value in this field is the seconds part of the update. When ADDSUB is reset, this field must be programmed with the seconds part of the update value. When ADDSUB is set, this field must be programmed with the complement of the seconds part of the update value. For example, to subtract 2.000000001 seconds from the system time, the TSS field in the ETH_MACSTSUR register must be 0xFFFF_FFFE (that is, 2^32 – 2). When the ADDSUB bit is set, TSSS\\[30:0\\]
field cannot be set to 0 in System time nanoseconds update register (ETH_MACSTNUR). The TSSS bitfield must be programmed to 0x7FFF FFFF (resulting in –0.46 ns) even if 0 ns must be subtracted. For example, to subtract 2.000000000 seconds from the system time, the TSS field in the System time seconds update register (ETH_MACSTSUR) must be 0xFFFF FFFE (that is, 2^32 – 1) and the System time nanoseconds update register (ETH_MACSTNUR) must be 0xFFFF FFFF (ADDSUB = 1 and TSSS\\[30:0\\]
field = 0x7FFF_FFFF)"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<MACSTSURrs> {
        TSS_W::new(self, 0)
    }
}
#[doc = "System time seconds update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstsur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macstsur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSTSURrs;
impl crate::RegisterSpec for MACSTSURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstsur::R`](R) reader structure"]
impl crate::Readable for MACSTSURrs {}
#[doc = "`write(|w| ..)` method takes [`macstsur::W`](W) writer structure"]
impl crate::Writable for MACSTSURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACSTSUR to value 0"]
impl crate::Resettable for MACSTSURrs {
    const RESET_VALUE: u32 = 0;
}
