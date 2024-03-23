#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Option register bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OR_ {
    #[doc = "0: Input 1 is connected to I/O"]
    Io = 0,
    #[doc = "1: Input 1 is connected to COMP1_OUT"]
    Comp1Out = 1,
    #[doc = "2: Input 1 is connected to COMP2_OUT"]
    Comp2Out = 2,
    #[doc = "3: Input 1 is connected to COMP1_OUT OR COMP2_OUT"]
    OrComp1Comp2 = 3,
}
impl From<OR_> for u8 {
    #[inline(always)]
    fn from(variant: OR_) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OR_ {
    type Ux = u8;
}
#[doc = "Field `OR_` reader - Option register bit 1"]
pub type OR__R = crate::FieldReader<OR_>;
impl OR__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OR_ {
        match self.bits {
            0 => OR_::Io,
            1 => OR_::Comp1Out,
            2 => OR_::Comp2Out,
            3 => OR_::OrComp1Comp2,
            _ => unreachable!(),
        }
    }
    #[doc = "Input 1 is connected to I/O"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == OR_::Io
    }
    #[doc = "Input 1 is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == OR_::Comp1Out
    }
    #[doc = "Input 1 is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == OR_::Comp2Out
    }
    #[doc = "Input 1 is connected to COMP1_OUT OR COMP2_OUT"]
    #[inline(always)]
    pub fn is_or_comp1_comp2(&self) -> bool {
        *self == OR_::OrComp1Comp2
    }
}
#[doc = "Field `OR_` writer - Option register bit 1"]
pub type OR__W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, OR_>;
impl<'a, REG> OR__W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input 1 is connected to I/O"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(OR_::Io)
    }
    #[doc = "Input 1 is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(OR_::Comp1Out)
    }
    #[doc = "Input 1 is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(OR_::Comp2Out)
    }
    #[doc = "Input 1 is connected to COMP1_OUT OR COMP2_OUT"]
    #[inline(always)]
    pub fn or_comp1_comp2(self) -> &'a mut crate::W<REG> {
        self.variant(OR_::OrComp1Comp2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Option register bit 1"]
    #[inline(always)]
    pub fn or_(&self) -> OR__R {
        OR__R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Option register bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn or_(&mut self) -> OR__W<ORrs> {
        OR__W::new(self, 0)
    }
}
#[doc = "option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for ORrs {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for ORrs {
    const RESET_VALUE: u32 = 0;
}
