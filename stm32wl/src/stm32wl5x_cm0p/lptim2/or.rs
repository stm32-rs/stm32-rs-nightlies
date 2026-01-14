///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
/**Option register bit 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OR_ {
    ///0: Input 1 is connected to I/O
    Io = 0,
    ///1: Input 1 is connected to COMP1_OUT
    Comp1Out = 1,
    ///2: Input 1 is connected to COMP2_OUT
    Comp2Out = 2,
    ///3: Input 1 is connected to COMP1_OUT OR COMP2_OUT
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
impl crate::IsEnum for OR_ {}
///Field `OR_` reader - Option register bit 1
pub type OR__R = crate::FieldReader<OR_>;
impl OR__R {
    ///Get enumerated values variant
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
    ///Input 1 is connected to I/O
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == OR_::Io
    }
    ///Input 1 is connected to COMP1_OUT
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == OR_::Comp1Out
    }
    ///Input 1 is connected to COMP2_OUT
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == OR_::Comp2Out
    }
    ///Input 1 is connected to COMP1_OUT OR COMP2_OUT
    #[inline(always)]
    pub fn is_or_comp1_comp2(&self) -> bool {
        *self == OR_::OrComp1Comp2
    }
}
///Field `OR_` writer - Option register bit 1
pub type OR__W<'a, REG> = crate::FieldWriter<'a, REG, 2, OR_, crate::Safe>;
impl<'a, REG> OR__W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input 1 is connected to I/O
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(OR_::Io)
    }
    ///Input 1 is connected to COMP1_OUT
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(OR_::Comp1Out)
    }
    ///Input 1 is connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(OR_::Comp2Out)
    }
    ///Input 1 is connected to COMP1_OUT OR COMP2_OUT
    #[inline(always)]
    pub fn or_comp1_comp2(self) -> &'a mut crate::W<REG> {
        self.variant(OR_::OrComp1Comp2)
    }
}
impl R {
    ///Bits 0:1 - Option register bit 1
    #[inline(always)]
    pub fn or_(&self) -> OR__R {
        OR__R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR").field("or_", &self.or_()).finish()
    }
}
impl W {
    ///Bits 0:1 - Option register bit 1
    #[inline(always)]
    pub fn or_(&mut self) -> OR__W<'_, ORrs> {
        OR__W::new(self, 0)
    }
}
/**option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#LPTIM2:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
