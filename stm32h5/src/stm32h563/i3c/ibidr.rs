///Register `IBIDR` reader
pub type R = crate::R<IBIDRrs>;
///Register `IBIDR` writer
pub type W = crate::W<IBIDRrs>;
///Field `IBIDB0` reader - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\[7:0\] mandatory data byte).
pub type IBIDB0_R = crate::FieldReader;
///Field `IBIDB0` writer - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\[7:0\] mandatory data byte).
pub type IBIDB0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IBIDB1` reader - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\[7:0\]).
pub type IBIDB1_R = crate::FieldReader;
///Field `IBIDB1` writer - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\[7:0\]).
pub type IBIDB1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IBIDB2` reader - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\[7:0\]).
pub type IBIDB2_R = crate::FieldReader;
///Field `IBIDB2` writer - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\[7:0\]).
pub type IBIDB2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IBIDB3` reader - 8-bit IBI payload data (latest byte on I3C bus).
pub type IBIDB3_R = crate::FieldReader;
///Field `IBIDB3` writer - 8-bit IBI payload data (latest byte on I3C bus).
pub type IBIDB3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\[7:0\] mandatory data byte).
    #[inline(always)]
    pub fn ibidb0(&self) -> IBIDB0_R {
        IBIDB0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\[7:0\]).
    #[inline(always)]
    pub fn ibidb1(&self) -> IBIDB1_R {
        IBIDB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\[7:0\]).
    #[inline(always)]
    pub fn ibidb2(&self) -> IBIDB2_R {
        IBIDB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - 8-bit IBI payload data (latest byte on I3C bus).
    #[inline(always)]
    pub fn ibidb3(&self) -> IBIDB3_R {
        IBIDB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBIDR")
            .field("ibidb0", &self.ibidb0())
            .field("ibidb1", &self.ibidb1())
            .field("ibidb2", &self.ibidb2())
            .field("ibidb3", &self.ibidb3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - 8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\[7:0\] mandatory data byte).
    #[inline(always)]
    pub fn ibidb0(&mut self) -> IBIDB0_W<'_, IBIDRrs> {
        IBIDB0_W::new(self, 0)
    }
    ///Bits 8:15 - 8-bit IBI payload data (next byte on I3C bus after IBIDB0\[7:0\]).
    #[inline(always)]
    pub fn ibidb1(&mut self) -> IBIDB1_W<'_, IBIDRrs> {
        IBIDB1_W::new(self, 8)
    }
    ///Bits 16:23 - 8-bit IBI payload data (next byte on I3C bus after IBIDB1\[7:0\]).
    #[inline(always)]
    pub fn ibidb2(&mut self) -> IBIDB2_W<'_, IBIDRrs> {
        IBIDB2_W::new(self, 16)
    }
    ///Bits 24:31 - 8-bit IBI payload data (latest byte on I3C bus).
    #[inline(always)]
    pub fn ibidb3(&mut self) -> IBIDB3_W<'_, IBIDRrs> {
        IBIDB3_W::new(self, 24)
    }
}
/**I3C IBI payload data register

You can [`read`](crate::Reg::read) this register and get [`ibidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#I3C:IBIDR)*/
pub struct IBIDRrs;
impl crate::RegisterSpec for IBIDRrs {
    type Ux = u32;
}
///`read()` method returns [`ibidr::R`](R) reader structure
impl crate::Readable for IBIDRrs {}
///`write(|w| ..)` method takes [`ibidr::W`](W) writer structure
impl crate::Writable for IBIDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IBIDR to value 0
impl crate::Resettable for IBIDRrs {}
