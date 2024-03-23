#[doc = "Register `CRR3` reader"]
pub type R = crate::R<CRR3rs>;
#[doc = "Register `CRR3` writer"]
pub type W = crate::W<CRR3rs>;
#[doc = "Field `BASEADDR` reader - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \\[28:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
pub type BASEADDR_R = crate::FieldReader;
#[doc = "Field `BASEADDR` writer - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \\[28:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
pub type BASEADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSIZE` reader - size for region x"]
pub type RSIZE_R = crate::FieldReader;
#[doc = "Field `RSIZE` writer - size for region x"]
pub type RSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REN` reader - enable for region x"]
pub type REN_R = crate::BitReader;
#[doc = "Field `REN` writer - enable for region x"]
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMAPADDR` reader - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \\[31:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
pub type REMAPADDR_R = crate::FieldReader<u16>;
#[doc = "Field `REMAPADDR` writer - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \\[31:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
pub type REMAPADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MSTSEL` reader - AHB cache master selection for region x"]
pub type MSTSEL_R = crate::BitReader;
#[doc = "Field `MSTSEL` writer - AHB cache master selection for region x"]
pub type MSTSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBURST` reader - output burst type for region x"]
pub type HBURST_R = crate::BitReader;
#[doc = "Field `HBURST` writer - output burst type for region x"]
pub type HBURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \\[28:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:11 - size for region x"]
    #[inline(always)]
    pub fn rsize(&self) -> RSIZE_R {
        RSIZE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 15 - enable for region x"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \\[31:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
    #[inline(always)]
    pub fn remapaddr(&self) -> REMAPADDR_R {
        REMAPADDR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - AHB cache master selection for region x"]
    #[inline(always)]
    pub fn mstsel(&self) -> MSTSEL_R {
        MSTSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - output burst type for region x"]
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \\[28:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn baseaddr(&mut self) -> BASEADDR_W<CRR3rs> {
        BASEADDR_W::new(self, 0)
    }
    #[doc = "Bits 9:11 - size for region x"]
    #[inline(always)]
    #[must_use]
    pub fn rsize(&mut self) -> RSIZE_W<CRR3rs> {
        RSIZE_W::new(self, 9)
    }
    #[doc = "Bit 15 - enable for region x"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<CRR3rs> {
        REN_W::new(self, 15)
    }
    #[doc = "Bits 16:26 - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \\[31:RI\\], where 21 ≤ RI ≤ 27 is the number of bits of RSIZE (see ). If the programmed value has more LSBs, the useless bits are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn remapaddr(&mut self) -> REMAPADDR_W<CRR3rs> {
        REMAPADDR_W::new(self, 16)
    }
    #[doc = "Bit 28 - AHB cache master selection for region x"]
    #[inline(always)]
    #[must_use]
    pub fn mstsel(&mut self) -> MSTSEL_W<CRR3rs> {
        MSTSEL_W::new(self, 28)
    }
    #[doc = "Bit 31 - output burst type for region x"]
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<CRR3rs> {
        HBURST_W::new(self, 31)
    }
}
#[doc = "ICACHE region 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRR3rs;
impl crate::RegisterSpec for CRR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crr3::R`](R) reader structure"]
impl crate::Readable for CRR3rs {}
#[doc = "`write(|w| ..)` method takes [`crr3::W`](W) writer structure"]
impl crate::Writable for CRR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRR3 to value 0x0200"]
impl crate::Resettable for CRR3rs {
    const RESET_VALUE: u32 = 0x0200;
}
