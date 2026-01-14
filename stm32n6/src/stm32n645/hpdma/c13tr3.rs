///Register `C13TR3` reader
pub type R = crate::R<C13TR3rs>;
///Register `C13TR3` writer
pub type W = crate::W<C13TR3rs>;
///Field `SAO` reader - source address offset increment
pub type SAO_R = crate::FieldReader<u16>;
///Field `SAO` writer - source address offset increment
pub type SAO_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `DAO` reader - destination address offset increment
pub type DAO_R = crate::FieldReader<u16>;
///Field `DAO` writer - destination address offset increment
pub type DAO_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - source address offset increment
    #[inline(always)]
    pub fn sao(&self) -> SAO_R {
        SAO_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - destination address offset increment
    #[inline(always)]
    pub fn dao(&self) -> DAO_R {
        DAO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C13TR3")
            .field("sao", &self.sao())
            .field("dao", &self.dao())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - source address offset increment
    #[inline(always)]
    pub fn sao(&mut self) -> SAO_W<'_, C13TR3rs> {
        SAO_W::new(self, 0)
    }
    ///Bits 16:28 - destination address offset increment
    #[inline(always)]
    pub fn dao(&mut self) -> DAO_W<'_, C13TR3rs> {
        DAO_W::new(self, 16)
    }
}
/**HPDMA channel 13 transfer register 3

You can [`read`](crate::Reg::read) this register and get [`c13tr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13tr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HPDMA:C13TR3)*/
pub struct C13TR3rs;
impl crate::RegisterSpec for C13TR3rs {
    type Ux = u32;
}
///`read()` method returns [`c13tr3::R`](R) reader structure
impl crate::Readable for C13TR3rs {}
///`write(|w| ..)` method takes [`c13tr3::W`](W) writer structure
impl crate::Writable for C13TR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C13TR3 to value 0
impl crate::Resettable for C13TR3rs {}
