#[doc = "Register `MACL3L4C1R` reader"]
pub type R = crate::R<MACL3L4C1Rrs>;
#[doc = "Register `MACL3L4C1R` writer"]
pub type W = crate::W<MACL3L4C1Rrs>;
#[doc = "Field `L3PEN1` reader - Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets. When this bit is reset, the Layer 3 IP Source or Destination Address matching is enabled for IPv4 packets. The Layer 3 matching is done only when the L3SAM1 or L3DAM1 bit is set."]
pub type L3PEN1_R = crate::BitReader;
#[doc = "Field `L3PEN1` writer - Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets. When this bit is reset, the Layer 3 IP Source or Destination Address matching is enabled for IPv4 packets. The Layer 3 matching is done only when the L3SAM1 or L3DAM1 bit is set."]
pub type L3PEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAM1` reader - Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Source Address field for matching. Note: When the L3PEN01 bit is set, you should set either this bit or the L3DAM1 bit because either IPv6 SA or DA can be checked for filtering."]
pub type L3SAM1_R = crate::BitReader;
#[doc = "Field `L3SAM1` writer - Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Source Address field for matching. Note: When the L3PEN01 bit is set, you should set either this bit or the L3DAM1 bit because either IPv6 SA or DA can be checked for filtering."]
pub type L3SAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAIM1` reader - Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching. When this bit reset, the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when the L3SAM1 bit is set."]
pub type L3SAIM1_R = crate::BitReader;
#[doc = "Field `L3SAIM1` writer - Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching. When this bit reset, the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when the L3SAM1 bit is set."]
pub type L3SAIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAM1` reader - Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When the L3PEN1 bit is set, you should set either this bit or the L3SAM1 bit because either IPv6 DA or SA can be checked for filtering."]
pub type L3DAM1_R = crate::BitReader;
#[doc = "Field `L3DAM1` writer - Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When the L3PEN1 bit is set, you should set either this bit or the L3SAM1 bit because either IPv6 DA or SA can be checked for filtering."]
pub type L3DAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAIM1` reader - Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching. When this bit is reset, the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when the L3DAM1 bit is set high."]
pub type L3DAIM1_R = crate::BitReader;
#[doc = "Field `L3DAIM1` writer - Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching. When this bit is reset, the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when the L3DAM1 bit is set high."]
pub type L3DAIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3HSBM1` reader - Layer 3 IP SA Higher Bits Match This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. This field contains Bits\\[4:0\\]
of L3HSBM1. These bits indicate the number of higher bits of IP Source or Destination Address matched in the IPv6 packets. This field is valid and applicable only when the L3DAM1 or L3SAM1 bit is set high."]
pub type L3HSBM1_R = crate::FieldReader;
#[doc = "Field `L3HSBM1` writer - Layer 3 IP SA Higher Bits Match This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. This field contains Bits\\[4:0\\]
of L3HSBM1. These bits indicate the number of higher bits of IP Source or Destination Address matched in the IPv6 packets. This field is valid and applicable only when the L3DAM1 or L3SAM1 bit is set high."]
pub type L3HSBM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L3HDBM1` reader - Layer 3 IP DA higher bits match This field contains the number of lower bits of IP Destination Address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. Bits\\[12:11\\]
of this field correspond to Bits\\[6:5\\]
of L3HSBM1, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 packets. The following list describes the concatenated values of the L3HDBM1\\[1:0\\]
and L3HSBM1 bits: .. This field is valid and applicable only when the L3DAM1 or L3SAM1 bit is set."]
pub type L3HDBM1_R = crate::FieldReader;
#[doc = "Field `L3HDBM1` writer - Layer 3 IP DA higher bits match This field contains the number of lower bits of IP Destination Address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. Bits\\[12:11\\]
of this field correspond to Bits\\[6:5\\]
of L3HSBM1, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 packets. The following list describes the concatenated values of the L3HDBM1\\[1:0\\]
and L3HSBM1 bits: .. This field is valid and applicable only when the L3DAM1 or L3SAM1 bit is set."]
pub type L3HDBM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L4PEN1` reader - Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching. When this bit is reset, the Source and Destination Port number fields of TCP packets are used for matching. The Layer 4 matching is done only when the L4SPM1 or L4DPM1 bit is set."]
pub type L4PEN1_R = crate::BitReader;
#[doc = "Field `L4PEN1` writer - Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching. When this bit is reset, the Source and Destination Port number fields of TCP packets are used for matching. The Layer 4 matching is done only when the L4SPM1 or L4DPM1 bit is set."]
pub type L4PEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPM1` reader - Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Source Port number field for matching."]
pub type L4SPM1_R = crate::BitReader;
#[doc = "Field `L4SPM1` writer - Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Source Port number field for matching."]
pub type L4SPM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPIM1` reader - Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4SPM1 bit is set high."]
pub type L4SPIM1_R = crate::BitReader;
#[doc = "Field `L4SPIM1` writer - Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4SPM1 bit is set high."]
pub type L4SPIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPM1` reader - Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
pub type L4DPM1_R = crate::BitReader;
#[doc = "Field `L4DPM1` writer - Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
pub type L4DPM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPIM1` reader - Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4DPM1 bit is set high."]
pub type L4DPIM1_R = crate::BitReader;
#[doc = "Field `L4DPIM1` writer - Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4DPM1 bit is set high."]
pub type L4DPIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets. When this bit is reset, the Layer 3 IP Source or Destination Address matching is enabled for IPv4 packets. The Layer 3 matching is done only when the L3SAM1 or L3DAM1 bit is set."]
    #[inline(always)]
    pub fn l3pen1(&self) -> L3PEN1_R {
        L3PEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Source Address field for matching. Note: When the L3PEN01 bit is set, you should set either this bit or the L3DAM1 bit because either IPv6 SA or DA can be checked for filtering."]
    #[inline(always)]
    pub fn l3sam1(&self) -> L3SAM1_R {
        L3SAM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching. When this bit reset, the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when the L3SAM1 bit is set."]
    #[inline(always)]
    pub fn l3saim1(&self) -> L3SAIM1_R {
        L3SAIM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When the L3PEN1 bit is set, you should set either this bit or the L3SAM1 bit because either IPv6 DA or SA can be checked for filtering."]
    #[inline(always)]
    pub fn l3dam1(&self) -> L3DAM1_R {
        L3DAM1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching. When this bit is reset, the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when the L3DAM1 bit is set high."]
    #[inline(always)]
    pub fn l3daim1(&self) -> L3DAIM1_R {
        L3DAIM1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - Layer 3 IP SA Higher Bits Match This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. This field contains Bits\\[4:0\\]
of L3HSBM1. These bits indicate the number of higher bits of IP Source or Destination Address matched in the IPv6 packets. This field is valid and applicable only when the L3DAM1 or L3SAM1 bit is set high."]
    #[inline(always)]
    pub fn l3hsbm1(&self) -> L3HSBM1_R {
        L3HSBM1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Layer 3 IP DA higher bits match This field contains the number of lower bits of IP Destination Address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. Bits\\[12:11\\]
of this field correspond to Bits\\[6:5\\]
of L3HSBM1, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 packets. The following list describes the concatenated values of the L3HDBM1\\[1:0\\]
and L3HSBM1 bits: .. This field is valid and applicable only when the L3DAM1 or L3SAM1 bit is set."]
    #[inline(always)]
    pub fn l3hdbm1(&self) -> L3HDBM1_R {
        L3HDBM1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching. When this bit is reset, the Source and Destination Port number fields of TCP packets are used for matching. The Layer 4 matching is done only when the L4SPM1 or L4DPM1 bit is set."]
    #[inline(always)]
    pub fn l4pen1(&self) -> L4PEN1_R {
        L4PEN1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Source Port number field for matching."]
    #[inline(always)]
    pub fn l4spm1(&self) -> L4SPM1_R {
        L4SPM1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4SPM1 bit is set high."]
    #[inline(always)]
    pub fn l4spim1(&self) -> L4SPIM1_R {
        L4SPIM1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
    #[inline(always)]
    pub fn l4dpm1(&self) -> L4DPM1_R {
        L4DPM1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4DPM1 bit is set high."]
    #[inline(always)]
    pub fn l4dpim1(&self) -> L4DPIM1_R {
        L4DPIM1_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Layer 3 Protocol Enable When this bit is set, the Layer 3 IP Source or Destination Address matching is enabled for IPv6 packets. When this bit is reset, the Layer 3 IP Source or Destination Address matching is enabled for IPv4 packets. The Layer 3 matching is done only when the L3SAM1 or L3DAM1 bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn l3pen1(&mut self) -> L3PEN1_W<MACL3L4C1Rrs> {
        L3PEN1_W::new(self, 0)
    }
    #[doc = "Bit 2 - Layer 3 IP SA Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Source Address field for matching. Note: When the L3PEN01 bit is set, you should set either this bit or the L3DAM1 bit because either IPv6 SA or DA can be checked for filtering."]
    #[inline(always)]
    #[must_use]
    pub fn l3sam1(&mut self) -> L3SAM1_W<MACL3L4C1Rrs> {
        L3SAM1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Layer 3 IP SA Inverse Match Enable When this bit is set, the Layer 3 IP Source Address field is enabled for inverse matching. When this bit reset, the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when the L3SAM1 bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn l3saim1(&mut self) -> L3SAIM1_W<MACL3L4C1Rrs> {
        L3SAIM1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Layer 3 IP DA Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for matching. When this bit is reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When the L3PEN1 bit is set, you should set either this bit or the L3SAM1 bit because either IPv6 DA or SA can be checked for filtering."]
    #[inline(always)]
    #[must_use]
    pub fn l3dam1(&mut self) -> L3DAM1_W<MACL3L4C1Rrs> {
        L3DAM1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Layer 3 IP DA Inverse Match Enable When this bit is set, the Layer 3 IP Destination Address field is enabled for inverse matching. When this bit is reset, the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when the L3DAM1 bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l3daim1(&mut self) -> L3DAIM1_W<MACL3L4C1Rrs> {
        L3DAIM1_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - Layer 3 IP SA Higher Bits Match This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. This field contains Bits\\[4:0\\]
of L3HSBM1. These bits indicate the number of higher bits of IP Source or Destination Address matched in the IPv6 packets. This field is valid and applicable only when the L3DAM1 or L3SAM1 bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l3hsbm1(&mut self) -> L3HSBM1_W<MACL3L4C1Rrs> {
        L3HSBM1_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - Layer 3 IP DA higher bits match This field contains the number of lower bits of IP Destination Address that are masked for matching in the IPv4 packets. The following list describes the values of this field: .. Bits\\[12:11\\]
of this field correspond to Bits\\[6:5\\]
of L3HSBM1, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 packets. The following list describes the concatenated values of the L3HDBM1\\[1:0\\]
and L3HSBM1 bits: .. This field is valid and applicable only when the L3DAM1 or L3SAM1 bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn l3hdbm1(&mut self) -> L3HDBM1_W<MACL3L4C1Rrs> {
        L3HDBM1_W::new(self, 11)
    }
    #[doc = "Bit 16 - Layer 4 Protocol Enable When this bit is set, the Source and Destination Port number fields of UDP packets are used for matching. When this bit is reset, the Source and Destination Port number fields of TCP packets are used for matching. The Layer 4 matching is done only when the L4SPM1 or L4DPM1 bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn l4pen1(&mut self) -> L4PEN1_W<MACL3L4C1Rrs> {
        L4PEN1_W::new(self, 16)
    }
    #[doc = "Bit 18 - Layer 4 Source Port Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Source Port number field for matching."]
    #[inline(always)]
    #[must_use]
    pub fn l4spm1(&mut self) -> L4SPM1_W<MACL3L4C1Rrs> {
        L4SPM1_W::new(self, 18)
    }
    #[doc = "Bit 19 - Layer 4 Source Port Inverse Match Enable When this bit is set, the Layer 4 Source Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4SPM1 bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l4spim1(&mut self) -> L4SPIM1_W<MACL3L4C1Rrs> {
        L4SPIM1_W::new(self, 19)
    }
    #[doc = "Bit 20 - Layer 4 Destination Port Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for matching. When this bit is reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
    #[inline(always)]
    #[must_use]
    pub fn l4dpm1(&mut self) -> L4DPM1_W<MACL3L4C1Rrs> {
        L4DPM1_W::new(self, 20)
    }
    #[doc = "Bit 21 - Layer 4 Destination Port Inverse Match Enable When this bit is set, the Layer 4 Destination Port number field is enabled for inverse matching. When this bit is reset, the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when the L4DPM1 bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l4dpim1(&mut self) -> L4DPIM1_W<MACL3L4C1Rrs> {
        L4DPIM1_W::new(self, 21)
    }
}
#[doc = "L3 and L4 control 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3l4c1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3l4c1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACL3L4C1Rrs;
impl crate::RegisterSpec for MACL3L4C1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3l4c1r::R`](R) reader structure"]
impl crate::Readable for MACL3L4C1Rrs {}
#[doc = "`write(|w| ..)` method takes [`macl3l4c1r::W`](W) writer structure"]
impl crate::Writable for MACL3L4C1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACL3L4C1R to value 0"]
impl crate::Resettable for MACL3L4C1Rrs {
    const RESET_VALUE: u32 = 0;
}
