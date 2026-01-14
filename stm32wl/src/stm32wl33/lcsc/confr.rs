///Register `CONFR` reader
pub type R = crate::R<CONFRrs>;
///Register `CONFR` writer
pub type W = crate::W<CONFRrs>;
///Field `CLKWISE_THRES` reader - Number of Clock Wise revolutions target
pub type CLKWISE_THRES_R = crate::FieldReader<u16>;
///Field `CLKWISE_THRES` writer - Number of Clock Wise revolutions target
pub type CLKWISE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ACLKWISE_THRES` reader - Number of Anti Clock Wise revolutions target
pub type ACLKWISE_THRES_R = crate::FieldReader<u16>;
///Field `ACLKWISE_THRES` writer - Number of Anti Clock Wise revolutions target
pub type ACLKWISE_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Number of Clock Wise revolutions target
    #[inline(always)]
    pub fn clkwise_thres(&self) -> CLKWISE_THRES_R {
        CLKWISE_THRES_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Number of Anti Clock Wise revolutions target
    #[inline(always)]
    pub fn aclkwise_thres(&self) -> ACLKWISE_THRES_R {
        ACLKWISE_THRES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFR")
            .field("clkwise_thres", &self.clkwise_thres())
            .field("aclkwise_thres", &self.aclkwise_thres())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Number of Clock Wise revolutions target
    #[inline(always)]
    pub fn clkwise_thres(&mut self) -> CLKWISE_THRES_W<'_, CONFRrs> {
        CLKWISE_THRES_W::new(self, 0)
    }
    ///Bits 16:31 - Number of Anti Clock Wise revolutions target
    #[inline(always)]
    pub fn aclkwise_thres(&mut self) -> ACLKWISE_THRES_W<'_, CONFRrs> {
        ACLKWISE_THRES_W::new(self, 16)
    }
}
/**LCSC_CONFR register

You can [`read`](crate::Reg::read) this register and get [`confr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:CONFR)*/
pub struct CONFRrs;
impl crate::RegisterSpec for CONFRrs {
    type Ux = u32;
}
///`read()` method returns [`confr::R`](R) reader structure
impl crate::Readable for CONFRrs {}
///`write(|w| ..)` method takes [`confr::W`](W) writer structure
impl crate::Writable for CONFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFR to value 0
impl crate::Resettable for CONFRrs {}
