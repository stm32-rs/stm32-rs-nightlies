///Register `COMP1_OR` reader
pub type R = crate::R<COMP1_ORrs>;
///Register `COMP1_OR` writer
pub type W = crate::W<COMP1_ORrs>;
///Field `AFOP` reader - Selection of source for alternate function of output ports
pub type AFOP_R = crate::FieldReader<u16>;
///Field `AFOP` writer - Selection of source for alternate function of output ports
pub type AFOP_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `OR` reader - Option Register
pub type OR_R = crate::FieldReader<u32>;
///Field `OR` writer - Option Register
pub type OR_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    ///Bits 0:10 - Selection of source for alternate function of output ports
    #[inline(always)]
    pub fn afop(&self) -> AFOP_R {
        AFOP_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:31 - Option Register
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP1_OR")
            .field("afop", &self.afop())
            .field("or", &self.or())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Selection of source for alternate function of output ports
    #[inline(always)]
    pub fn afop(&mut self) -> AFOP_W<'_, COMP1_ORrs> {
        AFOP_W::new(self, 0)
    }
    ///Bits 11:31 - Option Register
    #[inline(always)]
    pub fn or(&mut self) -> OR_W<'_, COMP1_ORrs> {
        OR_W::new(self, 11)
    }
}
/**Comparator option register

You can [`read`](crate::Reg::read) this register and get [`comp1_or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#COMP1:COMP1_OR)*/
pub struct COMP1_ORrs;
impl crate::RegisterSpec for COMP1_ORrs {
    type Ux = u32;
}
///`read()` method returns [`comp1_or::R`](R) reader structure
impl crate::Readable for COMP1_ORrs {}
///`write(|w| ..)` method takes [`comp1_or::W`](W) writer structure
impl crate::Writable for COMP1_ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP1_OR to value 0
impl crate::Resettable for COMP1_ORrs {}
